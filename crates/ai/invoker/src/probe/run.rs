use std::time::Duration;

use tokio::time::timeout;

use super::report::{self, MachineReport};
use super::{found, paths, script};
use crate::executor::{failure, run_script};
use crate::settings::check_machine;
use crate::times::PROBE_TIMEOUT;
use crate::{InvokerSettingsError, Machine};

/// ssh's exit code when it could not connect, as opposed to the command's.
const SSH_FAILED: i32 = 255;

/// Looks at the machine: whether it can be reached, what is installed where
/// its ProbeDirs say, which models are there. Goes through no Invoker, takes
/// no lock and starts no executor, so it runs beside calls. The machine may
/// be one not saved yet, so it is checked as saving would first: what goes
/// on the command line must be safe there. Without ProbeDirs it only sees
/// whether the machine answers.
pub async fn probe(
    machine: &Machine,
    connect_timeout: Duration,
) -> Result<MachineReport, InvokerSettingsError> {
    check_machine(machine)?;
    Ok(look(machine, connect_timeout).await)
}

async fn look(machine: &Machine, connect_timeout: Duration) -> MachineReport {
    let dirs = machine.probe.clone().unwrap_or_default();
    let files = report::files_to_check(&dirs);
    let script = script::script(&dirs.models, &files);
    let connect_timeout_secs = connect_timeout.as_secs().max(1);
    let mut tried: Vec<String> = Vec::new();
    // Tried in turn: one that is missing fails before it says anything.
    for python in paths::probe_pythons(&dirs) {
        let run = run_script(machine, &python, &script, connect_timeout_secs);
        let output = match timeout(PROBE_TIMEOUT, run).await {
            Err(_) => {
                return offline(machine, format!("{} 秒未响应", PROBE_TIMEOUT.as_secs()));
            }
            Ok(Err(reason)) => {
                tried.push(reason);
                continue;
            }
            Ok(Ok(output)) => output,
        };
        if let Some(found) = found::parse(&String::from_utf8_lossy(&output.stdout)) {
            log::info!("{}: probed with {python}", machine.name);
            return report::online(&dirs, &found);
        }
        if machine.ssh.is_some() && output.status.code() == Some(SSH_FAILED) {
            return offline(machine, format!("连接失败{}", failure(&output)));
        }
        tried.push(format!("{python}{}", failure(&output)));
    }
    offline(machine, format!("无可用的 python：{}", tried.join("；")))
}

fn offline(machine: &Machine, reason: String) -> MachineReport {
    log::warn!("{}: probe failed: {reason}", machine.name);
    MachineReport::Offline { reason }
}
