use serde::{Deserialize, Serialize};

use crate::{Progress, SenseKey, Sentence, Zone};

/// A sense in a zone.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub key: SenseKey,
    pub zone: Zone,
    /// Steps left until full.
    pub remaining: u32,
    pub glosses: Vec<String>,
    pub sentences: Vec<Sentence>,
    pub progress: Progress,
}
