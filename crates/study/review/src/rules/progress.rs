//! Progress written back on the ratings that change it before full.

use study_quiz::Progress;

/// Vague: one stage down, at least 0.
pub fn vague(progress: Progress) -> Progress {
    Progress {
        stage: progress.stage.saturating_sub(1),
        ..progress
    }
}

/// Unknown: back to stage 0, one more lapse.
pub fn unknown(progress: Progress) -> Progress {
    Progress {
        stage: 0,
        lapses: progress.lapses.saturating_add(1),
        ..progress
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROGRESS: Progress = Progress {
        stage: 3,
        passes: 2,
        lapses: 1,
    };

    #[test]
    fn vague_lowers_the_stage() {
        assert_eq!(vague(PROGRESS).stage, 2);
        assert_eq!(vague(Progress::default()).stage, 0);
    }

    #[test]
    fn unknown_restarts_and_counts_a_lapse() {
        assert_eq!(
            unknown(PROGRESS),
            Progress {
                stage: 0,
                passes: 2,
                lapses: 2
            }
        );
    }
}
