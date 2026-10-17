use chrono::{DateTime, FixedOffset};

/// An entry after validation and normalization, as nested layers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidEntry {
    pub writing: String,
    /// Empty for native words.
    pub origin: String,
    /// Empty when not given.
    pub stratum: String,
    pub reading: Option<ValidReading>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidReading {
    /// Hiragana.
    pub kana: String,
    pub sense: Option<ValidSense>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidSense {
    /// Non-empty and without duplicates; the first is the primary gloss.
    pub glosses: Vec<String>,
    pub labels: Vec<String>,
    /// Without duplicate plain texts.
    pub sentences: Vec<ValidSentence>,
    pub progress: Option<EntryProgress>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidSentence {
    pub marked: String,
    pub plain: String,
    pub translation: Option<String>,
}

/// Progress fields as given in the entry; missing counters are zero.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntryProgress {
    pub stage: u32,
    pub passes: u32,
    pub lapses: u32,
    pub modified_at: Option<DateTime<FixedOffset>>,
}
