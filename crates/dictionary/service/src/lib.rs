//! The dictionary group's public face: `DictionaryService` and `Settings`,
//! plus the sub-crate types that leave the group.

mod browse;
mod clock;
mod connection;
mod error;
mod log_access;
mod migrator;
mod render;
mod sense_access;
mod service;
mod settings;
mod settings_file;
mod transfer;

pub use error::DictionaryError;
pub use render::render_sentence;
pub use service::DictionaryService;
pub use settings::Settings;

pub use dictionary_export::ExportReport;
pub use dictionary_import::{ImportOptions, ImportProgress, ImportReport, ImportWarning};
pub use dictionary_log::{
    LogImportReport, LogPrediction, LogQuestionType, LogRating, LogRecord, LogSense,
};
pub use dictionary_word::{
    DictionaryStats, Fragment, SenseCandidate, SenseKey, SensePage, SenseSnapshot, SenseSummary,
    SentenceDisplay, SentenceSnapshot,
};
