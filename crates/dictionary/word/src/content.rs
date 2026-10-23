//! What the dictionary takes in and gives out: one word, written down to
//! some layer. Merging takes content in; dumping gives it out, one piece per
//! sense, or per word or reading that has nothing below it. The import
//! preview hands it to the frontend as is.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

/// A word, with at most one reading and one sense below it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct Content {
    pub writing: String,
    /// Empty for native words.
    pub origin: String,
    /// Empty when unknown.
    pub origin_language: String,
    /// `None` when the content stops at the word layer.
    pub reading: Option<ReadingContent>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct ReadingContent {
    /// Hiragana.
    pub kana: String,
    /// `None` when the content stops at the reading layer.
    pub sense: Option<SenseContent>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct SenseContent {
    /// Non-empty and without duplicates; the first is the primary gloss.
    pub glosses: Vec<String>,
    pub labels: Vec<String>,
    pub sentences: Vec<SentenceContent>,
    pub progress: Option<ContentProgress>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct SentenceContent {
    /// A valid marked sentence.
    pub marked: String,
    pub translation: Option<String>,
}

/// Progress carried by content. Without `modified_at` it cannot prove it is
/// newer than the progress already in the dictionary.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct ContentProgress {
    pub stage: u32,
    pub passes: u32,
    pub lapses: u32,
    pub modified_at: Option<DateTime<FixedOffset>>,
}
