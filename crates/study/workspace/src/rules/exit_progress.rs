use crate::Progress;

/// Progress written back when a sense becomes full: one more pass, one stage up (at most N).
pub fn full_progress(progress: Progress, max_stage: u32) -> Progress {
    Progress {
        stage: (progress.stage + 1).min(max_stage),
        passes: progress.passes + 1,
        lapses: progress.lapses,
    }
}

/// Progress written back when a sense is skipped: stage N.
pub fn skipped_progress(progress: Progress, max_stage: u32) -> Progress {
    Progress {
        stage: max_stage,
        ..progress
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_adds_a_pass_and_a_stage() {
        let progress = Progress {
            stage: 2,
            passes: 1,
            lapses: 4,
        };
        assert_eq!(
            full_progress(progress, 7),
            Progress {
                stage: 3,
                passes: 2,
                lapses: 4
            }
        );
        assert_eq!(
            full_progress(
                Progress {
                    stage: 7,
                    ..progress
                },
                7
            )
            .stage,
            7
        );
    }

    #[test]
    fn skip_jumps_to_max_stage() {
        let progress = Progress {
            stage: 2,
            passes: 1,
            lapses: 4,
        };
        assert_eq!(
            skipped_progress(progress, 7),
            Progress {
                stage: 7,
                passes: 1,
                lapses: 4
            }
        );
    }
}
