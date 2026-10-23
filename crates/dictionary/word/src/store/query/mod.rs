//! Reading the dictionary: rows matching a filter, a page of them, one
//! sense in full, counts, and which words are already there.

pub mod dump;
pub mod existing_words;
pub mod filter_senses;
pub mod search;
pub mod sense_detail;
pub mod stats;

mod content_filter;
mod row_data;
mod rows;
mod sense_condition;
mod sense_order;
mod text_condition;

pub use content_filter::{Completeness, ContentFilter, ProgressState};
pub use rows::{Row, matching_rows};
pub use sense_order::SenseOrder;
