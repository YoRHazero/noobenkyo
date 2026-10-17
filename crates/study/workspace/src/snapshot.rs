use serde::{Deserialize, Serialize};

use crate::{Progress, SenseKey};

/// Everything a zone keeps about a sense, taken when it enters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub key: SenseKey,
    /// All glosses, the primary gloss first.
    pub glosses: Vec<String>,
    pub sentences: Vec<Sentence>,
    pub progress: Progress,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sentence {
    pub marked: String,
    pub translation: Option<String>,
    /// Kana of the target as it appears in the sentence.
    pub target_kana: String,
}
