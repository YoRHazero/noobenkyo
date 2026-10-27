use std::time::Duration;

use tokio::time::{Instant, timeout};

use super::bundle::Bundle;
use super::command::CommandLine;
use super::deploy::deploy;
use super::{Executor, background};
use crate::Machine;
use crate::protocol::{StartRuntime, start_line};
use crate::times::{HEARTBEAT_TIMEOUT_SECS, READY_TIMEOUT};

/// What to start, where.
pub(crate) struct Launch<'a> {
    pub machine: &'a Machine,
    pub python: &'a str,
    pub model: &'a str,
    pub start: &'a StartRuntime,
    pub connect_timeout_secs: u64,
    pub idle_timeout: Duration,
}

/// Sends the code, starts the executor and waits for `ready`. On failure,
/// says why in words the user can read; nothing is left running.
pub(crate) async fn launch(launch: Launch<'_>, bundle: &Bundle) -> Result<Executor, String> {
    let started = Instant::now();
    let attempt = async {
        let dir = deploy(
            launch.machine,
            launch.python,
            bundle,
            launch.connect_timeout_secs,
        )
        .await?;
        let command = CommandLine::executor(
            launch.machine,
            launch.python,
            &dir,
            launch.connect_timeout_secs,
        );
        let child = command.spawn()?;
        let start = start_line(launch.start, HEARTBEAT_TIMEOUT_SECS);
        let (executor, ready) = background::spawn(
            &launch.machine.name,
            launch.model,
            child,
            start,
            launch.idle_timeout,
        );
        match ready.await {
            Ok(Ok(_)) => Ok(executor),
            Ok(Err(reason)) => Err(reason),
            Err(_) => Err("执行端没有说明就停下了".to_owned()),
        }
    };
    // Dropping the attempt drops the executor's remote control, which closes it.
    let result = timeout(READY_TIMEOUT, attempt)
        .await
        .unwrap_or_else(|_| Err(format!("{} 秒内没有就绪", READY_TIMEOUT.as_secs())));
    match &result {
        Ok(_) => log::info!(
            "{}: {} started in {:.1} s",
            launch.machine.name,
            launch.model,
            started.elapsed().as_secs_f64()
        ),
        Err(reason) => log::warn!(
            "{}: {} failed to start: {reason}",
            launch.machine.name,
            launch.model
        ),
    }
    result
}
