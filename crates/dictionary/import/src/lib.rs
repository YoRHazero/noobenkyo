//! Import: the entry format, validation, and layer-by-layer merging into
//! the word tables.

mod entry;
mod merge_outcome;
mod options;
mod report;
pub mod rules;
pub mod store;
mod valid_entry;

pub use entry::{Entry, EntrySentence};
pub use merge_outcome::{MergeOutcome, SenseOutcome};
pub use options::ImportOptions;
pub use report::{ImportProgress, ImportReport, ImportWarning};
pub use store::import_file::{ImportError, import_file};
pub use valid_entry::{EntryProgress, ValidEntry, ValidReading, ValidSense, ValidSentence};
