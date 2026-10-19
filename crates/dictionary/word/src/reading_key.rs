use serde::{Deserialize, Serialize};

/// Natural key of a reading.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
pub struct ReadingKey {
    pub writing: String,
    pub origin: String,
    pub kana: String,
}
