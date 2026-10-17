//! Test data shared by the workspace operations.

use chrono::NaiveDate;

use crate::rules::fixtures::key;
use crate::{Progress, Sentence, Snapshot};

pub fn today() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 10, 18).unwrap()
}

/// A snapshot with one sentence whose target is the writing.
pub fn snapshot(writing: &str, progress: Progress) -> Snapshot {
    Snapshot {
        key: key(writing),
        glosses: vec!["释义".into()],
        sentences: vec![Sentence {
            marked: format!("[[{writing}]]だ"),
            translation: Some("译文".into()),
            target_kana: "かな".into(),
        }],
        progress,
    }
}
