//! Basic reads and writes of each table, used by the queries of this crate.

/// Values per statement when loading rows by a list of ids or writings;
/// SQLite limits bound values.
const ID_CHUNK: usize = 500;

pub mod label;
pub mod reading;
pub mod sense;
pub mod sentence;
pub mod word;
