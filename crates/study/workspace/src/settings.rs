use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::LearningOrder;

/// Zone settings. Missing fields in the file take their defaults.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, specta::Type)]
#[serde(default)]
pub struct WorkspaceSettings {
    pub learning_capacity: u32,
    pub review_capacity: u32,
    pub fixed_group_size: u32,
    pub daily_new_limit: u32,
    pub learning_order: LearningOrder,
    /// Upper bound of k: senses asked in the last k questions are not picked.
    pub recent_limit: u32,
    /// Expected review interval in days for stages 0..=N; N is the last index.
    pub stage_intervals: Vec<f64>,
    /// Weight reached at the expected interval; before it the weight grows linearly up to this.
    pub early_growth: f64,
    /// Each pass lengthens the expected interval by this fraction.
    pub pass_bonus: f64,
    /// Share of each stage 0..=N in a fixed group.
    pub group_stage_ratios: Vec<f64>,
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
            stage_intervals: vec![1.0, 2.0, 4.0, 7.0, 15.0, 30.0, 60.0, 120.0],
            early_growth: 0.1,
            pass_bonus: 0.1,
            group_stage_ratios: vec![4.0, 4.0, 3.0, 3.0, 2.0, 2.0, 1.0, 1.0],
        }
    }
}

impl WorkspaceSettings {
    /// N: the highest stage.
    pub fn max_stage(&self) -> u32 {
        u32::try_from(self.stage_intervals.len().saturating_sub(1)).unwrap_or(0)
    }

    pub fn interval_for(&self, stage: u32) -> f64 {
        let index = usize::try_from(stage.min(self.max_stage())).unwrap_or(0);
        self.stage_intervals.get(index).copied().unwrap_or(1.0)
    }

    pub fn validate(&self) -> Result<(), WorkspaceSettingsError> {
        if self.learning_capacity == 0 || self.review_capacity == 0 || self.fixed_group_size == 0 {
            return Err(WorkspaceSettingsError::ZeroCapacity);
        }
        if self.stage_intervals.is_empty()
            || !self
                .stage_intervals
                .iter()
                .all(|d| d.is_finite() && *d > 0.0)
        {
            return Err(WorkspaceSettingsError::Intervals);
        }
        if !(self.early_growth.is_finite() && self.early_growth > 0.0) {
            return Err(WorkspaceSettingsError::EarlyGrowth);
        }
        if !(self.pass_bonus.is_finite() && self.pass_bonus >= 0.0) {
            return Err(WorkspaceSettingsError::PassBonus);
        }
        let ratios = &self.group_stage_ratios;
        if ratios.len() != self.stage_intervals.len()
            || !ratios.iter().all(|r| r.is_finite() && *r >= 0.0)
            || ratios.iter().sum::<f64>() <= 0.0
        {
            return Err(WorkspaceSettingsError::GroupRatios);
        }
        Ok(())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceSettingsError {
    #[error("容量和每组数量至少为 1")]
    ZeroCapacity,
    #[error("各档位的期望间隔必须是正数，至少一个档位")]
    Intervals,
    #[error("到期前的权重增长必须是正数")]
    EarlyGrowth,
    #[error("正确次数对间隔的加成不能为负")]
    PassBonus,
    #[error("定组的档位比例要和档位一一对应，不能为负，合计大于 0")]
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
        assert_eq!(settings.interval_for(99), 120.0);
    }

    #[test]
    fn rejects_mismatched_ratios() {
        let settings = WorkspaceSettings {
            group_stage_ratios: vec![1.0],
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
            stage_intervals: vec![1.0, 0.0],
            ..WorkspaceSettings::default()
        };
        assert_eq!(interval.validate(), Err(WorkspaceSettingsError::Intervals));
    }
}
