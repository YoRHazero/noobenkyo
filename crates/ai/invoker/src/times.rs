//! The fixed times from Invoker.md's table; the two that depend on the
//! user's machines are in the settings.

use std::time::Duration;

/// From the start of a launch (sending the code included) until `ready`.
/// Loading the 9B model takes 10 to 22 seconds; the first start after the
/// machine boots also starts WSL from a cold disk.
pub(crate) const READY_TIMEOUT: Duration = Duration::from_secs(180);

/// How often the executor hears from the app while it runs.
pub(crate) const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// The executor leaves after this long without a message: five missed pings.
pub(crate) const HEARTBEAT_TIMEOUT_SECS: u64 = 30;

/// How long a closing executor gets to exit before the local process is killed.
pub(crate) const EXIT_GRACE: Duration = Duration::from_secs(10);

/// From the start of a probe until its report. Probing loads nothing, but
/// the first connection after the machine boots also starts WSL.
pub(crate) const PROBE_TIMEOUT: Duration = Duration::from_secs(60);
