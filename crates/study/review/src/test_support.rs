//! Test data shared by the rule tests.

use chrono::{DateTime, Duration, FixedOffset};
use rand::SeedableRng;
use rand::rngs::StdRng;
use study_quiz::SenseKey;

use crate::Candidate;

pub fn rng() -> StdRng {
    StdRng::seed_from_u64(7)
}

pub fn now() -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339("2026-10-20T09:00:00+09:00").unwrap()
}

/// A candidate last modified `days_ago` days before `now()`.
pub fn candidate(writing: &str, stage: u32, days_ago: i64) -> Candidate {
    Candidate {
        key: SenseKey {
            writing: writing.into(),
            origin: String::new(),
            kana: "かな".into(),
            primary_gloss: "释义".into(),
        },
        stage,
        passes: 1,
        lapses: 0,
        modified_at: now() - Duration::days(days_ago),
    }
}
