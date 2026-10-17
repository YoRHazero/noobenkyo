use serde::Serialize;

/// Summary of one import.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, specta::Type)]
pub struct ImportReport {
    pub entries: u32,
    pub words_added: u32,
    pub readings_added: u32,
    pub senses_added: u32,
    pub senses_merged: u32,
    pub sentences_added: u32,
    pub progress_updated: u32,
    pub skipped_katakana: u32,
    pub warnings: Vec<ImportWarning>,
}

/// An entry that was not imported (or not fully), for the user to handle.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct ImportWarning {
    /// 1-based line number in the file.
    pub line: u32,
    pub writing: String,
    pub message: String,
}

/// Progress of a running import.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct ImportProgress {
    pub done: u32,
    pub total: u32,
}
