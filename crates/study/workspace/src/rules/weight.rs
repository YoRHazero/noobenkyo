//! Review weight from days since last review, stage and passes. It grows
//! slowly before the expected interval and without bound after it.

use crate::{Progress, WorkspaceSettings};

/// Keeps every candidate drawable.
const MIN_WEIGHT: f64 = 1e-6;

pub fn review_weight(days: f64, progress: &Progress, settings: &WorkspaceSettings) -> f64 {
    let interval = settings.interval_for(progress.stage)
        * (1.0 + settings.pass_bonus * f64::from(progress.passes));
    let ratio = days / interval;
    let weight = if ratio < 1.0 {
        settings.early_growth * ratio
    } else {
        settings.early_growth + (ratio - 1.0)
    };
    weight.max(MIN_WEIGHT)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn weight(days: f64, stage: u32, passes: u32) -> f64 {
        review_weight(
            days,
            &Progress {
                stage,
                passes,
                lapses: 0,
            },
            &WorkspaceSettings::default(),
        )
    }

    #[test]
    fn grows_slowly_before_interval_then_fast() {
        // Stage 2 expects 4 days; no passes.
        assert!((weight(2.0, 2, 0) - 0.05).abs() < 1e-9);
        assert!((weight(4.0, 2, 0) - 0.1).abs() < 1e-9);
        assert!((weight(12.0, 2, 0) - 2.1).abs() < 1e-9);
        assert!(weight(400.0, 2, 0) > 99.0);
    }

    #[test]
    fn higher_stage_and_more_passes_wait_longer() {
        assert!(weight(10.0, 1, 0) > weight(10.0, 3, 0));
        assert!(weight(10.0, 1, 0) > weight(10.0, 1, 10));
    }

    #[test]
    fn never_zero() {
        assert!(weight(0.0, 0, 0) > 0.0);
    }
}
