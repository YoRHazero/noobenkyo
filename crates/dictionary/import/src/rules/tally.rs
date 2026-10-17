use crate::{ImportReport, MergeOutcome, SenseOutcome};

/// Adds the counts of one outcome to the report. Conflicts are reported by the caller.
pub fn tally(report: &mut ImportReport, outcome: &MergeOutcome) {
    report.words_added += u32::from(outcome.word_added);
    report.readings_added += u32::from(outcome.reading_added);
    report.senses_added += u32::from(outcome.sense == SenseOutcome::Added);
    report.senses_merged += u32::from(outcome.sense == SenseOutcome::Merged);
    report.sentences_added += outcome.sentences_added;
    report.progress_updated += u32::from(outcome.progress_updated);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_up_outcomes() {
        let mut report = ImportReport::default();
        let added = MergeOutcome {
            word_added: true,
            reading_added: true,
            sense: SenseOutcome::Added,
            sentences_added: 2,
            progress_updated: false,
        };
        let merged = MergeOutcome {
            sense: SenseOutcome::Merged,
            sentences_added: 1,
            progress_updated: true,
            ..MergeOutcome::default()
        };
        tally(&mut report, &added);
        tally(&mut report, &merged);
        tally(
            &mut report,
            &MergeOutcome {
                sense: SenseOutcome::Conflict(vec![]),
                ..MergeOutcome::default()
            },
        );
        assert_eq!(
            (
                report.words_added,
                report.readings_added,
                report.senses_added,
                report.senses_merged
            ),
            (1, 1, 1, 1)
        );
        assert_eq!((report.sentences_added, report.progress_updated), (3, 1));
    }
}
