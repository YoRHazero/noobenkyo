use serde::{Deserialize, Serialize};

/// Conditions on the rows of the dictionary. A row is one piece of content:
/// a sense, or a reading or word with nothing below it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct ContentFilter {
    /// Contained in the writing, origin, kana or any gloss. Blank means no condition.
    pub text: Option<String>,
    /// The levels a row may be at; empty means any.
    pub completeness: Vec<Completeness>,
    pub progress: ProgressState,
}

/// How far a row is written. Every row is at exactly one level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum Completeness {
    /// Only the writing.
    WordOnly,
    /// Down to the reading.
    ReadingOnly,
    /// A sense without sentences.
    NoSentence,
    /// A sense with sentences: it can enter a zone.
    WithSentence,
}

/// Progress of a sense. Only senses have progress, so anything but `Any`
/// keeps sense rows only.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum ProgressState {
    #[default]
    Any,
    /// Zero passes and stage 0.
    Untouched,
    Touched,
}

impl ContentFilter {
    pub fn allows(&self, level: Completeness) -> bool {
        self.completeness.is_empty() || self.completeness.contains(&level)
    }

    /// The text condition, trimmed; `None` when blank.
    pub fn text(&self) -> Option<&str> {
        self.text
            .as_deref()
            .map(str::trim)
            .filter(|text| !text.is_empty())
    }
}
