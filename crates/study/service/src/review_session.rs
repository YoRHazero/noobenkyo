//! The review session: started with the current settings, ended by dropping it.

use rand::SeedableRng;
use rand::rngs::StdRng;
use study_review::{Candidate, ReviewService, SessionOptions};

use crate::{StudyError, StudyService, clock};

impl StudyService {
    /// Starts a session over the candidates, replacing any session there is.
    pub fn start_review(
        &mut self,
        candidates: Vec<Candidate>,
        options: SessionOptions,
    ) -> Result<(), StudyError> {
        options.validate()?;
        self.review = Some(ReviewService::start(
            candidates,
            options,
            self.settings.quiz.clone(),
            self.settings.review.clone(),
            clock::now(),
            StdRng::from_os_rng(),
        ));
        Ok(())
    }

    /// Ends the session; whatever was not written back is dropped.
    pub fn end_review(&mut self) {
        self.review = None;
    }

    /// The current session, if any.
    pub fn review(&mut self) -> Option<&mut ReviewService> {
        self.review.as_mut()
    }
}
