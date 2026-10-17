use crate::SenseKey;

/// Everything study needs about a sense while it sits in a zone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SenseSnapshot {
    pub key: SenseKey,
    /// All glosses, the primary gloss first.
    pub glosses: Vec<String>,
    pub sentences: Vec<SentenceSnapshot>,
    pub stage: u32,
    pub passes: u32,
    pub lapses: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SentenceSnapshot {
    pub marked: String,
    pub translation: Option<String>,
    /// Kana of the target as it appears in this sentence (possibly inflected).
    pub target_kana: String,
}
