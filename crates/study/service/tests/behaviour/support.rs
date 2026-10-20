//! Test data shared by the behaviour tests.

use chrono::Local;
use study::{Candidate, SenseKey};

pub fn key(writing: &str) -> SenseKey {
    SenseKey {
        writing: writing.into(),
        origin: String::new(),
        kana: "かな".into(),
        primary_gloss: "释义".into(),
    }
}

/// A learned sense at stage 1, modified just now.
pub fn candidate(writing: &str) -> Candidate {
    Candidate {
        key: key(writing),
        stage: 1,
        passes: 1,
        lapses: 0,
        modified_at: Local::now().fixed_offset(),
    }
}
