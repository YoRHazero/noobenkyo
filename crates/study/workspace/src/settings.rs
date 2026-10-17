use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::LearningOrder;

/// Zone settings.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, specta::Type)]
pub struct WorkspaceSettings {
    pub learning_capacity: u32,
    pub review_capacity: u32,
    pub fixed_group_size: u32,
    pub daily_new_limit: u32,
    pub learning_order: LearningOrder,
    /// Upper bound of k: senses asked in the last k questions are not picked.
    pub recent_limit: u32,
    /// Expected review interval in days for stages 0..=N; N is the last index.
    pub stage_intervals: Vec<u32>,
    /// Weight at the expected interval, in percent; before it the weight grows linearly up to this.
    pub due_weight_percent: u32,
    /// How much each pass lengthens the expected interval, in percent.
    pub pass_bonus_percent: u32,
    /// Share of each stage 0..=N in a fixed group.
    pub group_stage_ratios: Vec<u32>,
}

impl Default for WorkspaceSettings {
    fn default() -> Self {
        Self {
            learning_capacity: 10,
            review_capacity: 20,
            fixed_group_size: 30,
            daily_new_limit: 20,
            learning_order: LearningOrder::Creation,
            recent_limit: 2,
            stage_intervals: vec![1, 2, 4, 7, 15, 30, 60, 120],
            due_weight_percent: 10,
            pass_bonus_percent: 10,
            group_stage_ratios: vec![4, 4, 3, 3, 2, 2, 1, 1],
        }
    }
}

impl WorkspaceSettings {
    /// N: the highest stage.
    pub fn max_stage(&self) -> u32 {
        u32::try_from(self.stage_intervals.len().saturating_sub(1)).unwrap_or(0)
    }

    /// Expected interval in days for a stage; stages above N count as N.
    pub fn interval_for(&self, stage: u32) -> u32 {
        let index = usize::try_from(stage.min(self.max_stage())).unwrap_or(0);
        self.stage_intervals.get(index).copied().unwrap_or(1)
    }

    pub fn validate(&self) -> Result<(), WorkspaceSettingsError> {
        if self.learning_capacity == 0 || self.review_capacity == 0 || self.fixed_group_size == 0 {
            return Err(WorkspaceSettingsError::ZeroCapacity);
        }
        if self.stage_intervals.is_empty() || self.stage_intervals.contains(&0) {
            return Err(WorkspaceSettingsError::Intervals);
        }
        if self.due_weight_percent == 0 {
            return Err(WorkspaceSettingsError::DueWeight);
        }
        let ratios = &self.group_stage_ratios;
        if ratios.len() != self.stage_intervals.len() || ratios.iter().all(|r| *r == 0) {
            return Err(WorkspaceSettingsError::GroupRatios);
        }
        Ok(())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceSettingsError {
    #[error("容量和每组数量至少为 1")]
    ZeroCapacity,
    #[error("各档位的期望间隔至少 1 天，至少一个档位")]
    Intervals,
    #[error("到期时的权重至少为 1%")]
    DueWeight,
    #[error("定组的档位比例要和档位一一对应，且不能全为 0")]
    GroupRatios,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_valid() {
        let settings = WorkspaceSettings::default();
        assert_eq!(settings.validate(), Ok(()));
        assert_eq!(settings.max_stage(), 7);
        assert_eq!(settings.interval_for(99), 120);
    }

    #[test]
    fn rejects_mismatched_ratios() {
        let settings = WorkspaceSettings {
            group_stage_ratios: vec![1],
            ..WorkspaceSettings::default()
        };
        assert_eq!(
            settings.validate(),
            Err(WorkspaceSettingsError::GroupRatios)
        );
    }

    #[test]
    fn rejects_bad_numbers() {
        let zero = WorkspaceSettings {
            review_capacity: 0,
            ..WorkspaceSettings::default()
        };
        assert_eq!(zero.validate(), Err(WorkspaceSettingsError::ZeroCapacity));
        let interval = WorkspaceSettings {
            stage_intervals: vec![1, 0],
            ..WorkspaceSettings::default()
        };
        assert_eq!(interval.validate(), Err(WorkspaceSettingsError::Intervals));
    }
}
