use dictionary_word::Content;

use crate::{IgnoredField, Problem};

/// What an entry text converts to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Converted {
    /// Non-empty lines in the text.
    pub lines: u32,
    /// What can be imported, in text order.
    pub entries: Vec<ConvertedEntry>,
    /// What was dropped, by line.
    pub problems: Vec<Problem>,
    /// Unknown fields, by name.
    pub ignored_fields: Vec<IgnoredField>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConvertedEntry {
    pub line: u32,
    /// The writing as given, for problems found later.
    pub writing: String,
    pub content: Content,
}
