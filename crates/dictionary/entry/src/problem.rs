use serde::Serialize;

/// Something taken out of an entry text, and why.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct Problem {
    /// 1-based line number in the text.
    pub line: u32,
    /// The writing as given; empty when the line is not a valid entry object.
    pub writing: String,
    pub dropped: Dropped,
    /// Why, in Chinese. Shown to the user and handed to the model as is.
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Dropped {
    /// The whole entry.
    Entry,
    /// The origin's language, written without an origin; the rest of the
    /// entry is kept.
    OriginLanguage { value: String },
    /// One sentence; the rest of the entry is kept.
    Sentence { marked: String },
    /// The sense layer, when merging finds several matching senses; the word
    /// and reading are kept.
    Sense,
}

/// A field the format does not know, ignored wherever it appeared.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct IgnoredField {
    /// Field path, e.g. "pos", or "sentences.note" for a field of a sentence.
    pub name: String,
    /// The 1-based lines it appeared on, ascending.
    pub lines: Vec<u32>,
}
