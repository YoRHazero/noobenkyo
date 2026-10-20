//! Progress written back when a sense becomes full or is skipped, the same
//! in both lines.

use crate::Progress;

/// Full: one more pass, one stage up (at most N).
pub fn full(progress: Progress, max_stage: u32) -> Progress {
    Progress {
        stage: progress.stage.saturating_add(1).min(max_stage),
        passes: progress.passes.saturating_add(1),
        lapses: progress.lapses,
    }
}

/// Skipped: mastered, stage N.
pub fn skipped(progress: Progress, max_stage: u32) -> Progress {
    Progress {
        stage: max_stage,
        ..progress
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROGRESS: Progress = Progress {
        stage: 2,
        passes: 1,
        lapses: 4,
    };

    #[test]
    fn full_adds_a_pass_and_a_stage_up_to_n() {
        assert_eq!(
            full(PROGRESS, 7),
            Progress {
                stage: 3,
                passes: 2,
                lapses: 4
            }
        );
        assert_eq!(full(PROGRESS, 2).stage, 2);
    }

    #[test]
    fn skip_sets_stage_n() {
        assert_eq!(
            skipped(PROGRESS, 7),
            Progress {
                stage: 7,
                ..PROGRESS
            }
        );
    }
}
