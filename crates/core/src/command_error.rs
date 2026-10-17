use serde::Serialize;

/// An error shown to the user.
#[derive(Debug, Serialize, specta::Type, thiserror::Error)]
#[error("{message}")]
pub struct CommandError {
    pub message: String,
}

impl From<dictionary::DictionaryError> for CommandError {
    fn from(error: dictionary::DictionaryError) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}

impl From<study::StudyError> for CommandError {
    fn from(error: study::StudyError) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}
