//! Rating, skipping and reporting. Exits are settled by the caller, who then
//! calls `remove`.

use crate::{
    RateOutcome, Rating, ReportedSentence, Settlement, StudyError, StudyService, Zone, clock,
};

impl StudyService {
    pub fn rate(&mut self, zone: Zone, rating: Rating) -> Result<RateOutcome, StudyError> {
        let outcome = self.workspace.rate(
            zone,
            rating,
            &self.settings.quiz,
            &self.settings.workspace,
            clock::now(),
        )?;
        self.persist()?;
        Ok(outcome)
    }

    pub fn skip(&mut self, zone: Zone) -> Result<Settlement, StudyError> {
        Ok(self.workspace.skip(zone, &self.settings.workspace)?)
    }

    pub fn report_sentence(&mut self, zone: Zone) -> Result<ReportedSentence, StudyError> {
        Ok(self.workspace.report_sentence(zone)?)
    }
}
