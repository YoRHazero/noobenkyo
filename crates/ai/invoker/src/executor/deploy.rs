//! Sends the executor's code to the machine before each start. The script
//! skips the unpacking when that version is already there.

use super::bundle::Bundle;
use super::script::{failure, run_script};
use crate::Machine;
use crate::settings::is_safe_on_command_line;

/// The absolute path of the code's directory on the machine.
pub(crate) async fn deploy(
    machine: &Machine,
    python: &str,
    bundle: &Bundle,
    connect_timeout_secs: u64,
) -> Result<String, String> {
    let output = run_script(
        machine,
        python,
        bundle.install_script(),
        connect_timeout_secs,
    )
    .await?;
    if !output.status.success() {
        return Err(format!("送代码失败{}", failure(&output)));
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
