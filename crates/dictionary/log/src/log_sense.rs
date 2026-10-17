use serde::{Deserialize, Serialize};

/// The sense of a record, stored as text rather than a reference.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogSense {
    #[serde(rename = "写法")]
    pub writing: String,
    #[serde(rename = "词源")]
    pub origin: String,
    #[serde(rename = "读音")]
    pub kana: String,
    #[serde(rename = "主释义")]
    pub primary_gloss: String,
}
