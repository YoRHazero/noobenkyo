use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{SessionOptions, SessionOptionsError};

/// Settings of the review line only; the shared ones are in `QuizSettings`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct ReviewSettings {
    /// k: days the expected interval grows by per stage.
    pub interval_per_stage: u32,
    /// b: the expected interval at stage 0, in days.
    pub interval_base: u32,
    /// The queue holds at most this many senses. More than `recent_limit`, so
    /// a full queue always has a sense not asked lately.
    pub queue_limit: u32,
    /// Upper bound of k: senses asked in the last k quizzes are left out.
    pub recent_limit: u32,
    /// What the start dialog offers.
    pub defaults: SessionOptions,
}

impl Default for ReviewSettings {
    fn default() -> Self {
        Self {
            interval_per_stage: 3,
            interval_base: 1,
            queue_limit: 20,
            recent_limit: 8,
            defaults: SessionOptions::default(),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ReviewSettingsError {
    #[error("档位 0 的期望间隔至少 1 天")]
    IntervalBase,
    #[error("复习队列的上限要大于排除最近几次的上限")]
    QueueLimit,
    #[error("{0}")]
    Defaults(#[from] SessionOptionsError),
}

impl ReviewSettings {
    /// Expected interval in days: k × stage + b.
    pub fn interval(&self, stage: u32) -> f64 {
        f64::from(self.interval_per_stage) * f64::from(stage) + f64::from(self.interval_base)
    }

    pub fn validate(&self) -> Result<(), ReviewSettingsError> {
        if self.interval_base == 0 {
            return Err(ReviewSettingsError::IntervalBase);
        }
        if self.queue_limit <= self.recent_limit {
            return Err(ReviewSettingsError::QueueLimit);
        }
        Ok(self.defaults.validate()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_grows_linearly_with_stage() {
        let settings = ReviewSettings::default();
        assert_eq!(settings.validate(), Ok(()));
        assert_eq!(settings.interval(0), 1.0);
        assert_eq!(settings.interval(2), 7.0);
    }

    #[test]
    fn rejects_zero_base_and_a_queue_within_the_recent_limit() {
        let base = ReviewSettings {
            interval_base: 0,
            ..ReviewSettings::default()
        };
        assert_eq!(base.validate(), Err(ReviewSettingsError::IntervalBase));
        let queue = ReviewSettings {
            queue_limit: 2,
            recent_limit: 2,
            ..ReviewSettings::default()
        };
        assert_eq!(queue.validate(), Err(ReviewSettingsError::QueueLimit));
        let bigger = ReviewSettings {
            queue_limit: 3,
            ..queue
        };
        assert_eq!(bigger.validate(), Ok(()));
    }
}
