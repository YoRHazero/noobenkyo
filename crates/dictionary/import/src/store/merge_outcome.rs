use crate::ImportReport;

/// What merging one entry changed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MergeOutcome {
    pub word_added: bool,
    pub reading_added: bool,
    pub sense: SenseOutcome,
    pub sentences_added: u32,
    pub progress_updated: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum SenseOutcome {
    /// The entry stops above the sense layer.
    #[default]
    Absent,
    Added,
    Merged,
    /// Glosses intersect these senses (by primary gloss); nothing was merged.
    Conflict(Vec<String>),
}

/// Adds the counts of one outcome to the report. Conflicts are reported by the caller.
pub fn tally(report: &mut ImportReport, outcome: &MergeOutcome) {
    report.words_added += u32::from(outcome.word_added);
    report.readings_added += u32::from(outcome.reading_added);
    report.senses_added += u32::from(outcome.sense == SenseOutcome::Added);
    report.senses_merged += u32::from(outcome.sense == SenseOutcome::Merged);
    report.sentences_added += outcome.sentences_added;
    report.progress_updated += u32::from(outcome.progress_updated);
}
