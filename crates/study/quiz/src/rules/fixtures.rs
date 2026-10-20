//! Shared test data.

use crate::SenseKey;

pub fn key(writing: &str) -> SenseKey {
    SenseKey {
        writing: writing.into(),
        origin: String::new(),
        kana: "あきらめる".into(),
        primary_gloss: "放弃".into(),
    }
}
