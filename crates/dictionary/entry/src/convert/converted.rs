use dictionary_word::Content;
use serde::Serialize;

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

/// One entry and the content it converts to.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct ConvertedEntry {
    /// 1-based line number in the text.
    pub line: u32,
    /// The writing as given, for problems found later.
    pub writing: String,
    pub content: Content,
}
