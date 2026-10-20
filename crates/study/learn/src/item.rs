use serde::{Deserialize, Serialize};
use study_quiz::SenseKey;

/// A sense in the learning zone.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub key: SenseKey,
    /// Steps left until full.
    pub remaining: u32,
}
