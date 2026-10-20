use serde::{Deserialize, Serialize};
use study_learn::LearnSettings;
use study_quiz::QuizSettings;
use study_review::ReviewSettings;

use crate::StudyError;

/// Study settings: the shared part and each line's own part.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct StudySettings {
    pub quiz: QuizSettings,
    pub learn: LearnSettings,
    pub review: ReviewSettings,
}

impl StudySettings {
    pub fn validate(&self) -> Result<(), StudyError> {
        let invalid =
            |error: &dyn std::fmt::Display| StudyError::InvalidSettings(error.to_string());
        self.quiz.validate().map_err(|e| invalid(&e))?;
        self.learn.validate().map_err(|e| invalid(&e))?;
        self.review.validate().map_err(|e| invalid(&e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_every_part() {
        assert!(StudySettings::default().validate().is_ok());
        let mut settings = StudySettings::default();
        settings.review.queue_limit = 0;
        assert!(matches!(
            settings.validate(),
            Err(StudyError::InvalidSettings(_))
        ));
    }
}
