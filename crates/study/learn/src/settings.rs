use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Settings of the learning line only; the shared ones are in `QuizSettings`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct LearnSettings {
    /// How many senses the learning zone holds.
    pub capacity: u32,
    /// The order new senses enter in.
    pub order: LearnOrder,
    /// Upper bound of k: senses asked in the last k quizzes are left out.
    pub recent_limit: u32,
}

/// The order new senses enter the learning zone in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum LearnOrder {
    /// Creation (rowid) order.
    #[default]
    Creation,
    Random,
}

impl Default for LearnSettings {
    fn default() -> Self {
        Self {
            capacity: 40,
            order: LearnOrder::Creation,
            recent_limit: 8,
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LearnSettingsError {
    #[error("学习区的容量至少为 1")]
    Capacity,
}

impl LearnSettings {
    pub fn validate(&self) -> Result<(), LearnSettingsError> {
        if self.capacity == 0 {
            return Err(LearnSettingsError::Capacity);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn needs_room_for_one() {
        assert_eq!(LearnSettings::default().validate(), Ok(()));
        let empty = LearnSettings {
            capacity: 0,
            ..LearnSettings::default()
        };
        assert_eq!(empty.validate(), Err(LearnSettingsError::Capacity));
    }
}
