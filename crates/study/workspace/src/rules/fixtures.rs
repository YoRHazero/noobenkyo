//! Shared test data.

use chrono::{DateTime, Duration, FixedOffset};
use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::{Candidate, Progress, SenseKey};

pub fn rng() -> StdRng {
    StdRng::seed_from_u64(7)
}

pub fn now() -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339("2026-10-18T09:00:00+08:00").unwrap()
}

pub fn key(writing: &str) -> SenseKey {
    SenseKey {
        writing: writing.into(),
        origin: String::new(),
        kana: "かな".into(),
        primary_gloss: "释义".into(),
    }
}

/// A candidate last modified `days_ago` days before `now()`.
pub fn candidate(writing: &str, stage: u32, passes: u32, days_ago: i64) -> Candidate {
    Candidate {
        key: key(writing),
        progress: Progress {
            stage,
            passes,
            lapses: 0,
        },
        modified_at: now() - Duration::days(days_ago),
    }
}
