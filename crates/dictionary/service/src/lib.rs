//! The dictionary group's public face: `DictionaryService`, the pure
//! functions `check_entries` and `render_sentence`, and the sub-crate types
//! that leave the group. Every input and output is a value; files the user
//! picks are read and written by the caller.

mod clock;
mod connection;
mod delete;
mod edit;
mod entries;
mod error;
mod query;
mod render;
mod service;

pub use entries::check_entries;
pub use error::DictionaryError;
pub use render::render_sentence;
pub use service::DictionaryService;

pub use dictionary_entry::{Dropped, Exported, IgnoredField, ImportOptions, ImportReport, Problem};
pub use dictionary_word::{
    Completeness, ContentFilter, DictionaryStats, Fragment, Progress, ProgressState, ReadingKey,
    Replaced, SearchPage, SearchRow, SenseDetail, SenseKey, SenseOrder, SensePatch, SenseProgress,
    SenseSummary, SentenceContent, SentenceDisplay, StageCount, TargetPiece, WordKey,
};
