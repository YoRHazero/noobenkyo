use serde::{Deserialize, Serialize};

/// Natural key of a word.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
pub struct WordKey {
    pub writing: String,
    pub origin: String,
}
