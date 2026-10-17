use serde::{Deserialize, Serialize};

/// Natural key of a sense. Outside the dictionary a sense is only ever
/// identified by this key, never by its row id.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
pub struct SenseKey {
    pub writing: String,
    pub origin: String,
    pub kana: String,
    pub primary_gloss: String,
}
