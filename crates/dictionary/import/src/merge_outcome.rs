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
