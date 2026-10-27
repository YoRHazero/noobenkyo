use std::pin::pin;
use std::sync::{Mutex, MutexGuard, PoisonError};

use tokio::sync::Notify;

use super::lease::Lease;
use super::pick::{Availability, pick};
use crate::executor::Executor;

/// Each machine's state, behind a short lock never held across an await,
/// and the signal that a machine has come free.
pub(crate) struct State {
    machines: Mutex<Vec<MachineState>>,
    changed: Notify,
}

#[derive(Default)]
pub(crate) struct MachineState {
    pub busy: bool,
    /// May have died since: check `is_alive()`.
    pub executor: Option<Executor>,
}

impl State {
    pub(crate) fn new(machines: usize) -> Self {
        Self {
            machines: Mutex::new((0..machines).map(|_| MachineState::default()).collect()),
            changed: Notify::new(),
        }
    }

    /// Waits until one of the candidates is free and takes it.
    pub(crate) async fn lease(&self, candidates: &[usize], model: &str) -> Lease<'_> {
        loop {
            // Registered before looking, so a release in between still wakes us.
            let mut changed = pin!(self.changed.notified());
            changed.as_mut().enable();
            if let Some(machine) = self.try_take(candidates, model) {
                return Lease::new(self, machine);
            }
            changed.await;
        }
    }

    fn try_take(&self, candidates: &[usize], model: &str) -> Option<usize> {
        let mut machines = self.lock();
        let available: Vec<Availability> = machines
            .iter()
            .map(|machine| Availability {
                busy: machine.busy,
                loaded: machine
                    .executor
                    .as_ref()
                    .filter(|executor| executor.is_alive())
                    .map(Executor::model),
            })
            .collect();
        let machine = pick(&available, candidates, model)?;
        machines[machine].busy = true;
        Some(machine)
    }

    pub(super) fn release(&self, machine: usize) {
        self.lock()[machine].busy = false;
        self.changed.notify_waiters();
    }

    pub(super) fn lock(&self) -> MutexGuard<'_, Vec<MachineState>> {
        // Nothing panics while holding it; recover all the same.
        self.machines.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
