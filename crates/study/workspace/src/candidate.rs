use chrono::{DateTime, FixedOffset};

use crate::{Progress, SenseKey};

/// A sense with at least one sentence, offered for entry into a zone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Candidate {
    pub key: SenseKey,
    pub progress: Progress,
    pub modified_at: DateTime<FixedOffset>,
}
