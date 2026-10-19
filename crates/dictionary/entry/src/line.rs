use std::collections::BTreeMap;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

/// One line of an entry text, as written. Only `writing` is required;
/// deeper fields require the layers above them.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
    /// Fields the format does not know. They are ignored and never written.
    #[serde(flatten, skip_serializing)]
    pub unknown: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EntrySentence {
    pub marked: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translation: Option<String>,
    /// Fields the format does not know. They are ignored and never written.
    #[serde(flatten, skip_serializing)]
    pub unknown: BTreeMap<String, serde_json::Value>,
}

impl Entry {
    /// Paths of the unknown fields, e.g. "pos" or "sentences.note", each once.
    pub fn unknown_fields(&self) -> Vec<String> {
        let mut names: Vec<String> = self.unknown.keys().cloned().collect();
        for sentence in self.sentences.iter().flatten() {
            for key in sentence.unknown.keys() {
                let name = format!("sentences.{key}");
                if !names.contains(&name) {
                    names.push(name);
                }
            }
        }
        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_unknown_fields_and_never_writes_them() {
        let entry: Entry = serde_json::from_str(
            r#"{"writing": "明日", "pos": "名词", "sentences": [
                {"marked": "[[{明日|あした}]]", "note": 1}, {"marked": "[[{明日|あす}]]", "note": 2}]}"#,
        )
        .unwrap();
        assert_eq!(entry.unknown_fields(), ["pos", "sentences.note"]);
        assert_eq!(
            serde_json::to_string(&entry).unwrap(),
            r#"{"writing":"明日","sentences":[{"marked":"[[{明日|あした}]]"},{"marked":"[[{明日|あす}]]"}]}"#
        );
    }
}
