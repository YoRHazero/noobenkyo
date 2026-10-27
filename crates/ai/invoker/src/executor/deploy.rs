//! Sends the executor's code to the machine before each start. The script
//! skips the unpacking when that version is already there.

use tokio::io::AsyncWriteExt;

use super::bundle::Bundle;
use super::command::CommandLine;
use super::exit;
use super::stderr_tail::StderrTail;
use crate::Machine;
use crate::settings::is_safe_on_command_line;

/// The absolute path of the code's directory on the machine.
pub(crate) async fn deploy(
    machine: &Machine,
    python: &str,
    bundle: &Bundle,
    connect_timeout_secs: u64,
) -> Result<String, String> {
    let command = CommandLine::install(machine, python, connect_timeout_secs);
    let mut child = command.spawn()?;
    let mut stdin = child.stdin.take().expect("stdin is piped");
    let script = bundle.install_script().as_bytes();
    // Written while the output is read, so neither side waits on a full pipe.
    // A failed write shows up as the process failing, with the reason on stderr.
    let write = async move {
        let _ = stdin.write_all(script).await;
    };
    let (_, output) = tokio::join!(write, child.wait_with_output());
    let output = output.map_err(|error| format!("送代码失败：{error}"))?;
    if !output.status.success() {
        let mut tail = StderrTail::default();
        for line in String::from_utf8_lossy(&output.stderr).lines() {
            tail.push(line.to_owned());
        }
        return Err(format!(
            "送代码失败（{}）{}",
            exit::describe(output.status),
            tail.summary()
        ));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let dir = stdout
        .lines()
        .map(str::trim)
        .rfind(|line| !line.is_empty())
        .unwrap_or_default();
    if !dir.starts_with('/') || !dir.ends_with(bundle.hash()) {
        return Err(format!("送代码后没有拿到代码的目录：{dir:?}"));
    }
    if !is_safe_on_command_line(dir) {
        return Err(format!("代码的目录放不上命令行：{dir}"));
    }
    Ok(dir.to_owned())
}
