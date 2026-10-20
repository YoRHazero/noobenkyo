use chrono::{DateTime, FixedOffset};
use study_quiz::SenseKey;

/// A learned sense with sentences, offered when a session starts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Candidate {
    pub key: SenseKey,
    pub stage: u32,
    pub passes: u32,
    pub lapses: u32,
    pub modified_at: DateTime<FixedOffset>,
}
