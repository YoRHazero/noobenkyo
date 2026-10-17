//! Asking questions. The current question lives only in memory.

use crate::{Question, Reveal, StudyError, StudyService, Zone};

impl StudyService {
    /// The zone's current question, choosing one if needed. `None` when the zone is empty.
    pub fn current_question(&mut self, zone: Zone) -> Option<Question> {
        self.workspace.current_question(
            zone,
            &self.settings.quiz,
            &self.settings.workspace,
            &mut self.rng,
        )
    }

    /// Records the input and reveals the answer with a prediction.
    pub fn submit(&mut self, zone: Zone, input: String) -> Result<Reveal, StudyError> {
        Ok(self.workspace.submit(zone, input)?)
    }
}
