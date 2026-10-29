//! Running a script on a machine with `python -`, the script sent on stdin,
//! and waiting for it to end: how the executor's code is put in place, and
//! how a machine is probed.

use std::process::Output;

use tokio::io::AsyncWriteExt;

use super::command::CommandLine;
use super::exit;
use super::stderr_tail::StderrTail;
use crate::Machine;

/// Fails only when the process could not be started or waited for; a
/// script that fails still gives its output.
pub(crate) async fn run_script(
    machine: &Machine,
    python: &str,
    script: &str,
    connect_timeout_secs: u64,
) -> Result<Output, String> {
    let command = CommandLine::script(machine, python, connect_timeout_secs);
    let mut child = command.spawn()?;
    let mut stdin = child.stdin.take().expect("stdin is piped");
    let script = script.as_bytes();
    // Written while the output is read, so neither side waits on a full pipe.
    // A failed write shows up as the process failing, with the reason on stderr.
    let write = async move {
        let _ = stdin.write_all(script).await;
    };
    let (_, output) = tokio::join!(write, child.wait_with_output());
    output.map_err(|error| format!("等 {} 结束失败：{error}", command.program))
}

/// How a script that failed ended, and the last of what it wrote to stderr:
/// `（退出码 1）：…`.
pub(crate) fn failure(output: &Output) -> String {
    let mut tail = StderrTail::default();
    for line in String::from_utf8_lossy(&output.stderr).lines() {
        tail.push(line.to_owned());
    }
    format!("（{}）{}", exit::describe(output.status), tail.summary())
}
