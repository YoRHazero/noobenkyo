//! The workspace: persisted zone state plus in-memory question state.
//! Operations are split by concern across the files of this module.

mod answering;
mod asking;
mod entry;
mod exiting;
mod status;

use crate::WorkspaceState;
use crate::session::Session;

pub struct Workspace {
    state: WorkspaceState,
    session: Session,
}

impl Workspace {
    pub fn new(state: WorkspaceState) -> Self {
        Self {
            state,
            session: Session::default(),
        }
    }

    /// The state to persist.
    pub fn state(&self) -> &WorkspaceState {
        &self.state
    }

    fn position(&self, zone: crate::Zone, key: &crate::SenseKey) -> Option<usize> {
        self.state
            .items
            .iter()
            .position(|item| item.zone == zone && &item.key == key)
    }
}
