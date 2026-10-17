use crate::{Progress, Zone};

/// The zone a sense may enter: learning when it has no passes and stage 0, review otherwise.
pub fn source_zone(progress: &Progress) -> Zone {
    if progress.passes == 0 && progress.stage == 0 {
        Zone::Learning
    } else {
        Zone::Review
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_by_passes_and_stage() {
        assert_eq!(
            source_zone(&Progress {
                stage: 0,
                passes: 0,
                lapses: 3
            }),
            Zone::Learning
        );
        assert_eq!(
            source_zone(&Progress {
                stage: 1,
                passes: 0,
                lapses: 0
            }),
            Zone::Review
        );
        assert_eq!(
            source_zone(&Progress {
                stage: 0,
                passes: 1,
                lapses: 0
            }),
            Zone::Review
        );
    }
}
