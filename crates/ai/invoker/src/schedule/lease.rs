use super::state::State;
use crate::executor::Executor;

/// A machine taken for one call. Dropping it frees the machine and wakes
/// the calls waiting for one.
pub(crate) struct Lease<'a> {
    state: &'a State,
    machine: usize,
}

impl<'a> Lease<'a> {
    pub(super) fn new(state: &'a State, machine: usize) -> Self {
        Self { state, machine }
    }

    /// Index of the machine in the settings.
    pub(crate) fn machine(&self) -> usize {
        self.machine
    }

    /// The machine's executor, if it is alive and has `model` loaded.
    pub(crate) fn executor_for(&self, model: &str) -> Option<Executor> {
        self.state.lock()[self.machine]
            .executor
            .as_ref()
            .filter(|executor| executor.is_alive() && executor.model() == model)
            .cloned()
    }

    /// Whatever executor the machine has, alive or not, leaving none.
    pub(crate) fn take_executor(&self) -> Option<Executor> {
        self.state.lock()[self.machine].executor.take()
    }

    pub(crate) fn set_executor(&self, executor: Executor) {
        self.state.lock()[self.machine].executor = Some(executor);
    }
}

impl Drop for Lease<'_> {
    fn drop(&mut self) {
        self.state.release(self.machine);
    }
}
