//! Senses leaving the zones, the review mode, and zone status.

use crate::{ReviewMode, SenseKey, Settlement, StudyError, StudyService, Zone, ZoneStatus, clock};

impl StudyService {
    /// Removes a sense from its zone after its settlement is written or discarded.
    pub fn remove(&mut self, zone: Zone, key: &SenseKey) -> Result<(), StudyError> {
        self.workspace.remove(zone, key);
        self.persist()
    }

    /// Settlements to write before switching the review mode.
    pub fn review_settlements(&self) -> Vec<Settlement> {
        self.workspace.review_settlements()
    }

    pub fn review_mode(&self) -> ReviewMode {
        self.workspace.review_mode()
    }

    /// Switches the review mode, emptying the review zone.
    pub fn set_review_mode(&mut self, mode: ReviewMode) -> Result<(), StudyError> {
        self.workspace.set_review_mode(mode);
        self.persist()
    }

    pub fn status(&self, zone: Zone) -> ZoneStatus {
        self.workspace
            .status(zone, &self.settings.workspace, clock::today())
    }
}
