//! Reads and writes of the word tables. Functions take any connection, so
//! callers can run them inside a transaction.

pub mod browse;
pub mod candidates;
pub mod label;
pub mod progress;
pub mod reading;
pub mod sense;
pub mod sense_by_key;
pub mod sentence;
pub mod sentence_removal;
pub mod snapshot;
pub mod stats;
pub mod word;

#[cfg(test)]
pub(crate) mod test_db;
