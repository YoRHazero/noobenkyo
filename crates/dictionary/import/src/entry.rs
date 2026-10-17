use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

/// One line of an import file. Only `writing` is required; deeper fields
/// require the layers above them.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    pub writing: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stratum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kana: Option<String>,
    /// The first gloss is the primary gloss.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glosses: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sentences: Option<Vec<EntrySentence>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passes: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lapses: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<DateTime<FixedOffset>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntrySentence {
    pub marked: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translation: Option<String>,
}
