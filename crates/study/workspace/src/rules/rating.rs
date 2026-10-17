use study_answer::Rating;

use crate::{Progress, Zone};

/// Remaining steps and progress after a self-rating.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rated {
    pub remaining: u32,
    pub progress: Progress,
}

pub fn apply_rating(
    remaining: u32,
    progress: Progress,
    rating: Rating,
    zone: Zone,
    steps: u32,
) -> Rated {
    match rating {
        Rating::Known => Rated {
            remaining: remaining.saturating_sub(1),
            progress,
        },
        Rating::Vague => Rated {
            remaining: (remaining + 1).min(steps),
            progress: Progress {
                stage: progress.stage.saturating_sub(1),
                ..progress
            },
        },
        Rating::Unknown => Rated {
            remaining: steps,
            progress: Progress {
                stage: 0,
                lapses: progress.lapses + u32::from(zone == Zone::Review),
                ..progress
            },
        },
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
    fn known_takes_one_step() {
        assert_eq!(
            apply_rating(2, PROGRESS, Rating::Known, Zone::Review, 4),
            Rated {
                remaining: 1,
                progress: PROGRESS
            }
        );
    }

    #[test]
    fn vague_adds_a_step_and_lowers_stage() {
        assert_eq!(
            apply_rating(2, PROGRESS, Rating::Vague, Zone::Review, 4),
            Rated {
                remaining: 3,
                progress: Progress {
                    stage: 2,
                    ..PROGRESS
                }
            }
        );
        assert_eq!(
            apply_rating(4, PROGRESS, Rating::Vague, Zone::Learning, 4).remaining,
            4
        );
        let bottom = Progress {
            stage: 0,
            ..PROGRESS
        };
        assert_eq!(
            apply_rating(1, bottom, Rating::Vague, Zone::Review, 4)
                .progress
                .stage,
            0
        );
    }

    #[test]
    fn unknown_restarts_and_counts_lapse_only_in_review() {
        assert_eq!(
            apply_rating(1, PROGRESS, Rating::Unknown, Zone::Review, 4),
            Rated {
                remaining: 4,
                progress: Progress {
                    stage: 0,
                    passes: 2,
                    lapses: 2
                }
            }
        );
        assert_eq!(
            apply_rating(1, PROGRESS, Rating::Unknown, Zone::Learning, 4)
                .progress
                .lapses,
            1
        );
    }
}
