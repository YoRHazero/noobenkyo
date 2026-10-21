//! The dictionary group's public face: `DictionaryService`, the pure
//! functions `preview_entries`, `check_entries` and `render_sentence`, and
//! the sub-crate types that leave the group. Every input and output is a
//! value; files the user picks are read and written by the caller.

mod clock;
mod connection;
mod delete;
mod edit;
mod entries;
mod error;
mod query;
mod render;
mod service;

pub use entries::{check_entries, preview_entries};
pub use error::DictionaryError;
pub use render::render_sentence;
pub use service::DictionaryService;

pub use dictionary_entry::{
    ConvertedEntry, Dropped, Exported, IgnoredField, ImportOptions, ImportPreview, ImportReport,
    Problem,
};
pub use dictionary_word::{
    Completeness, Content, ContentFilter, ContentProgress, DictionaryStats, Fragment, Progress,
    ProgressState, ReadingContent, ReadingKey, Replaced, SearchPage, SearchRow, SenseContent,
    SenseDetail, SenseKey, SenseOrder, SensePatch, SenseProgress, SenseSummary, SentenceContent,
    SentenceDisplay, StageCount, TargetPiece, WordKey,
};
