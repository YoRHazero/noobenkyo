use chrono::{DateTime, FixedOffset};

/// Learning progress of a sense. Any change to it updates `modified_at`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Progress {
    pub stage: u32,
    pub passes: u32,
    pub lapses: u32,
    pub modified_at: DateTime<FixedOffset>,
}
