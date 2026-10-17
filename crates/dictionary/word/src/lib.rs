//! Words, readings, senses, labels and sentences: the schema, the generated
//! entities, storage, and the marked-text format of sentences.

#[rustfmt::skip]
pub mod entity;
mod migration;
pub mod rules;
pub mod store;

mod candidate;
mod fragment;
mod progress;
mod sense_key;
mod snapshot;

pub use candidate::SenseCandidate;
pub use fragment::{Fragment, SentenceDisplay};
pub use migration::migrations;
pub use progress::Progress;
pub use sense_key::SenseKey;
pub use snapshot::{SenseSnapshot, SentenceSnapshot};
pub use store::browse::{SensePage, SenseSummary};
pub use store::stats::DictionaryStats;
