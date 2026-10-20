//! Reads and writes of the word tables. Functions take any connection, so
//! callers can run them inside a transaction.

pub mod delete;
pub mod edit;
pub mod merge;
pub mod query;

mod by_key;
mod tables;
