//! Basic reads and writes of each table, used by the queries of this crate.

/// Ids per statement when loading rows by id; SQLite limits bound values.
const ID_CHUNK: usize = 500;

pub mod label;
pub mod reading;
pub mod sense;
pub mod sentence;
pub mod word;
