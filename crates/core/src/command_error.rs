use serde::Serialize;

/// The one error the frontend sees: a message in Chinese, as the feature it
/// came from wrote it.
#[derive(Debug, Serialize, specta::Type, thiserror::Error)]
#[error("{message}")]
pub struct CommandError {
    pub message: String,
}

impl CommandError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl From<dictionary::DictionaryError> for CommandError {
    fn from(error: dictionary::DictionaryError) -> Self {
        Self::new(error.to_string())
    }
}

impl From<study::StudyError> for CommandError {
    fn from(error: study::StudyError) -> Self {
        Self::new(error.to_string())
    }
}

impl From<study::LearnError> for CommandError {
    fn from(error: study::LearnError) -> Self {
        Self::new(error.to_string())
    }
}

impl From<study::AnswerError> for CommandError {
    fn from(error: study::AnswerError) -> Self {
        Self::new(error.to_string())
    }
}
