use serde::{Deserialize, Serialize};

/// Natural key of a sense. Study's own type; core converts it one-to-one.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SenseKey {
    pub writing: String,
    pub origin: String,
    pub kana: String,
    pub primary_gloss: String,
}
