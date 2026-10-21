use serde::{Deserialize, Serialize};
use thiserror::Error;

/// How a session starts: chosen by the user when starting, with defaults
/// from the settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct SessionOptions {
    /// Senses in a group; `None` for an endless session.
    pub size: Option<u32>,
    /// Steps a sense from the pool starts with, counted from the end of its
    /// question order.
    pub start_steps: u32,
    pub order: PickOrder,
}

/// How the pool is ordered.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum PickOrder {
    /// The most overdue first.
    #[default]
    Urgent,
    /// At random, more likely the more overdue.
    Weighted,
}

impl Default for SessionOptions {
    fn default() -> Self {
        Self {
            size: Some(20),
            start_steps: 1,
            order: PickOrder::Urgent,
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SessionOptionsError {
    #[error("每组至少 1 个")]
    Size,
    #[error("起始步数至少为 1")]
    StartSteps,
}

impl SessionOptions {
    pub fn validate(&self) -> Result<(), SessionOptionsError> {
        if self.size == Some(0) {
            return Err(SessionOptionsError::Size);
        }
        if self.start_steps == 0 {
            return Err(SessionOptionsError::StartSteps);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_groups_and_zero_steps() {
        assert_eq!(SessionOptions::default().validate(), Ok(()));
        let endless = SessionOptions {
            size: None,
            ..SessionOptions::default()
        };
        assert_eq!(endless.validate(), Ok(()));
        let empty = SessionOptions {
            size: Some(0),
            ..SessionOptions::default()
        };
        assert_eq!(empty.validate(), Err(SessionOptionsError::Size));
        let still = SessionOptions {
            start_steps: 0,
            ..SessionOptions::default()
        };
        assert_eq!(still.validate(), Err(SessionOptionsError::StartSteps));
    }
}
