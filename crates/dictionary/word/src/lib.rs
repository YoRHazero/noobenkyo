//! The dictionary itself: words, readings, senses, labels and sentences.
//! The schema, the generated entities, the rules of text and of merging, and
//! every read and write of the tables.

#[rustfmt::skip]
pub mod entity;
mod migration;
pub mod rules;
pub mod store;

mod content;
mod progress;
mod reading_key;
mod sense_key;
mod word_key;

pub use content::{Content, ContentProgress, ReadingContent, SenseContent, SentenceContent};
pub use migration::Migrator;
pub use progress::Progress;
pub use reading_key::ReadingKey;
pub use rules::sentence::{Fragment, SentenceDisplay, TargetPiece};
pub use sense_key::SenseKey;
pub use store::edit::{Replaced, SensePatch};
pub use store::merge::{MergeOutcome, SenseOutcome};
pub use store::query::filter_senses::SenseProgress;
pub use store::query::search::{SearchPage, SearchRow, SenseSummary};
pub use store::query::sense_detail::SenseDetail;
pub use store::query::stats::DictionaryStats;
pub use store::query::{Completeness, ContentFilter, ProgressState, SenseOrder};
pub use word_key::WordKey;
