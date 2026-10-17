use serde::{Deserialize, Serialize};
use study_quiz::QuizSettings;
use study_workspace::WorkspaceSettings;

use crate::StudyError;

/// Study settings: the configuration parameters of the design.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, specta::Type)]
#[serde(rename = "StudySettings")]
pub struct Settings {
    pub quiz: QuizSettings,
    pub workspace: WorkspaceSettings,
}

impl Settings {
    pub fn validate(&self) -> Result<(), StudyError> {
        self.quiz
            .validate()
            .map_err(|e| StudyError::InvalidSettings(e.to_string()))?;
        self.workspace
            .validate()
            .map_err(|e| StudyError::InvalidSettings(e.to_string()))
    }
}
