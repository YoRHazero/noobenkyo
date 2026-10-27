//! One task per executor, holding its local process (ssh, or the executor
//! itself when there is no ssh): writes requests, cancels and pings to its
//! stdin, hands each message from its stdout to the call it belongs to,
//! logs its stderr, and closes it after it sits idle. It never touches the
//! invoker's state: when the task ends, the remote control's channel closes,
//! and `is_alive()` is all anyone sees of it.

mod routes;

use std::io;
use std::process::ExitStatus;
use std::time::Duration;

use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWriteExt, BufReader, Split};
use tokio::process::{Child, ChildStderr, ChildStdin, ChildStdout};
use tokio::sync::{mpsc, oneshot};
use tokio::time::{Instant, MissedTickBehavior, timeout};

use self::routes::Routes;
use super::exit;
use super::stderr_tail::StderrTail;
use super::{Executor, Reply};
use crate::protocol::{Incoming, Versions, cancel_line, ping_line};
use crate::times::{EXIT_GRACE, HEARTBEAT_INTERVAL};

/// What the remote control asks of the task.
pub(crate) enum Command {
    Request {
        id: u64,
        line: String,
        replies: mpsc::UnboundedSender<Reply>,
    },
    Cancel {
        id: u64,
    },
    Shutdown {
        done: oneshot::Sender<()>,
    },
}

/// Answered once: the versions on `ready`, or why it never got there.
pub(super) type Ready = oneshot::Receiver<Result<Versions, String>>;

/// Starts the task for a freshly spawned process and sends it `start`.
pub(super) fn spawn(
    machine: &str,
    model: &str,
    mut child: Child,
    start: String,
    idle_timeout: Duration,
) -> (Executor, Ready) {
    let (commands, receiver) = mpsc::unbounded_channel();
    let (ready, ready_receiver) = oneshot::channel();
    let task = Task {
        machine: machine.to_owned(),
        model: model.to_owned(),
        stdin: child.stdin.take(),
        stdout: child
            .stdout
            .take()
            .map(|out| BufReader::new(out).split(b'\n')),
        stderr: child
            .stderr
            .take()
            .map(|err| BufReader::new(err).split(b'\n')),
        child,
        commands: receiver,
        routes: Routes::default(),
        ready: Some(ready),
        tail: StderrTail::default(),
        started: Instant::now(),
        idle_timeout,
        idle_since: Instant::now(),
        shutdowns: Vec::new(),
    };
    tokio::spawn(task.run(start));
    (Executor::new(model, commands), ready_receiver)
}

struct Task {
    machine: String,
    model: String,
    child: Child,
    /// None once closed, or once a write has failed.
    stdin: Option<ChildStdin>,
    /// None once at the end.
    stdout: Option<Split<BufReader<ChildStdout>>>,
    stderr: Option<Split<BufReader<ChildStderr>>>,
    commands: mpsc::UnboundedReceiver<Command>,
    routes: Routes,
    /// Some until `ready`, or an error before it, has come.
    ready: Option<oneshot::Sender<Result<Versions, String>>>,
    tail: StderrTail,
    started: Instant,
    idle_timeout: Duration,
    /// When the last request ended, or when it became ready.
    idle_since: Instant,
    /// Remote controls waiting for the process to have exited.
    shutdowns: Vec<oneshot::Sender<()>>,
}

/// Why the task stops serving.
enum Ending {
    /// Asked to: a shutdown, idle too long, or no remote control left.
    Closed(&'static str),
    /// Its stdout reached the end: the process is gone.
    Exited,
    /// It wrote something that is not the protocol.
    Broken(String),
}

impl Task {
    async fn run(mut self, start: String) {
        self.write(&start).await;
        let ending = self.serve().await;
        self.finish(ending).await;
    }

    async fn serve(&mut self) -> Ending {
        let mut heartbeat =
            tokio::time::interval_at(Instant::now() + HEARTBEAT_INTERVAL, HEARTBEAT_INTERVAL);
        heartbeat.set_missed_tick_behavior(MissedTickBehavior::Delay);
        loop {
            let idle = self.ready.is_none() && self.routes.is_empty();
            let idle_deadline = self.idle_since + self.idle_timeout;
            tokio::select! {
                command = self.commands.recv() => match command {
                    None => return Ending::Closed("没有人再用它"),
                    Some(Command::Shutdown { done }) => {
                        self.shutdowns.push(done);
                        return Ending::Closed("要关掉它");
                    }
                    Some(Command::Request { id, line, replies }) => {
                        self.routes.add(id, replies);
                        self.write(&line).await;
                    }
                    Some(Command::Cancel { id }) => {
                        if self.routes.cancel(id) {
                            self.write(&cancel_line(id)).await;
                        }
                    }
                },
                line = next_line(&mut self.stdout) => match line {
                    Some(line) => {
                        if let Err(error) = self.take(&line) {
                            return Ending::Broken(error);
                        }
                    }
                    None => return Ending::Exited,
                },
                line = next_line(&mut self.stderr) => self.log_stderr(line),
                _ = heartbeat.tick() => self.write(&ping_line()).await,
                _ = tokio::time::sleep_until(idle_deadline), if idle => {
                    return Ending::Closed("闲置超时");
                }
            }
        }
    }

    /// Handles one message from the executor.
    fn take(&mut self, line: &[u8]) -> Result<(), String> {
        let message = Incoming::parse(line).map_err(|error| {
            let shown = String::from_utf8_lossy(&line[..line.len().min(200)]).into_owned();
            format!("{error}：{shown}")
        })?;
        match message {
            Incoming::Ready { versions } => {
                let ready = self.ready.take().ok_or("ready came twice")?;
                let listed: Vec<String> =
                    versions.iter().map(|(k, v)| format!("{k} {v}")).collect();
                log::info!(
                    "{}: {} ready {:.1} s after start ({})",
                    self.machine,
                    self.model,
                    self.started.elapsed().as_secs_f64(),
                    listed.join(", ")
                );
                let _ = ready.send(Ok(versions));
                self.idle_since = Instant::now();
            }
            Incoming::Progress { id, tokens } => match self.routes.get(id) {
                Some(route) => {
                    let _ = route.replies.send(Reply::Progress(tokens));
                }
                None => log::warn!("{}: progress for unknown request {id}", self.machine),
            },
            Incoming::Done { id, fields } => self.end(id, Some(Reply::Done(fields))),
            Incoming::Cancelled { id } => self.end(id, None),
            Incoming::Error {
                id: Some(id),
                message,
            } => self.end(id, Some(Reply::Error(message))),
            Incoming::Error { id: None, message } => match self.ready.take() {
                Some(ready) => {
                    let _ = ready.send(Err(message));
                }
                None => log::warn!("{}: executor error: {message}", self.machine),
            },
        }
        Ok(())
    }

    fn end(&mut self, id: u64, reply: Option<Reply>) {
        let Some(route) = self.routes.take(id) else {
            log::warn!("{}: answer for unknown request {id}", self.machine);
            return;
        };
        if let Some(at) = route.cancelled_at {
            log::info!(
                "{}: request {id} stopped {} ms after its cancel",
                self.machine,
                at.elapsed().as_millis()
            );
        }
        if let Some(reply) = reply {
            let _ = route.replies.send(reply);
        }
        if self.routes.is_empty() {
            self.idle_since = Instant::now();
        }
    }

    async fn write(&mut self, line: &str) {
        let Some(stdin) = &mut self.stdin else {
            return;
        };
        let mut bytes = Vec::with_capacity(line.len() + 1);
        bytes.extend_from_slice(line.as_bytes());
        bytes.push(b'\n');
        if let Err(error) = stdin.write_all(&bytes).await {
            // The process is going; its stdout ending will say so.
            log::warn!("{}: cannot write to the executor: {error}", self.machine);
            self.stdin = None;
        }
    }

    fn log_stderr(&mut self, line: Option<Vec<u8>>) {
        let Some(line) = line else {
            self.stderr = None;
            return;
        };
        let text = String::from_utf8_lossy(&line).trim_end().to_owned();
        log::info!("{}: {text}", self.machine);
        self.tail.push(text);
    }

    /// Closes stdin, waits for the process to exit, and tells everyone
    /// still waiting on it.
    async fn finish(mut self, ending: Ending) {
        self.commands.close();
        while let Ok(command) = self.commands.try_recv() {
            match command {
                Command::Request { replies, .. } => {
                    let _ = replies.send(Reply::Gone);
                }
                Command::Shutdown { done } => self.shutdowns.push(done),
                Command::Cancel { .. } => {}
            }
        }
        // EOF on stdin is how the executor learns to leave.
        self.stdin = None;
        let status = self.wait_for_exit().await;
        let reason = match ending {
            Ending::Closed(why) => format!("执行端已关闭（{why}）"),
            Ending::Exited => format!("执行端退出了（{status}）{}", self.tail.summary()),
            Ending::Broken(error) => format!("执行端的消息不对：{error}"),
        };
        log::info!(
            "{}: {} closed, {status}: {reason}",
            self.machine,
            self.model
        );
        if let Some(ready) = self.ready.take() {
            let _ = ready.send(Err(reason.clone()));
        }
        self.routes.fail_all(&reason);
        for done in self.shutdowns.drain(..) {
            let _ = done.send(());
        }
    }

    async fn wait_for_exit(&mut self) -> String {
        let status = match timeout(EXIT_GRACE, self.drain_until_exit()).await {
            Ok(status) => status,
            Err(_) => {
                log::warn!(
                    "{}: not gone {} s after stdin closed, killing it",
                    self.machine,
                    EXIT_GRACE.as_secs()
                );
                let _ = self.child.start_kill();
                self.child.wait().await
            }
        };
        // What it wrote last may still be in the pipe.
        let _ = timeout(Duration::from_secs(1), async {
            while self.stderr.is_some() {
                let line = next_line(&mut self.stderr).await;
                self.log_stderr(line);
            }
        })
        .await;
        match status {
            Ok(status) => exit::describe(status),
            Err(error) => error.to_string(),
        }
    }

    /// Reads both pipes while waiting, so the process never blocks on a full one.
    async fn drain_until_exit(&mut self) -> io::Result<ExitStatus> {
        loop {
            tokio::select! {
                status = self.child.wait() => return status,
                line = next_line(&mut self.stdout) => {
                    if line.is_none() {
                        self.stdout = None;
                    }
                }
                line = next_line(&mut self.stderr) => self.log_stderr(line),
            }
        }
    }
}

/// The next line without its newline; None at the end. Never resolves once
/// the pipe is gone, so a select! can keep it as a branch.
async fn next_line<R: AsyncBufRead + Unpin>(lines: &mut Option<Split<R>>) -> Option<Vec<u8>> {
    let Some(lines) = lines else {
        return std::future::pending().await;
    };
    let mut line = lines.next_segment().await.ok().flatten()?;
    if line.last() == Some(&b'\r') {
        line.pop();
    }
    Some(line)
}
