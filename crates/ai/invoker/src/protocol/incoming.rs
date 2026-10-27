use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::{Map, Value};

use crate::LlmFinish;

/// Versions an executor reports when ready; only logged.
pub(crate) type Versions = BTreeMap<String, String>;

/// A message from an executor.
#[derive(Debug, PartialEq)]
pub(crate) enum Incoming {
    Ready {
        versions: Versions,
    },
    Progress {
        id: u64,
        tokens: u32,
    },
    /// The fields depend on the call, so the call that knows which one it
    /// made checks them (`LlmDone`, `TtsDone`).
    Done {
        id: u64,
        fields: Map<String, Value>,
    },
    Cancelled {
        id: u64,
    },
    /// No id: the executor failed before it was ready.
    Error {
        id: Option<u64>,
        message: String,
    },
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum Fixed {
    Ready { versions: Versions },
    Progress { id: u64, tokens: u32 },
    Cancelled { id: u64 },
    Error { id: Option<u64>, message: String },
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub(crate) struct LlmDone {
    pub text: String,
    pub tokens: u32,
    pub finish: LlmFinish,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub(crate) struct TtsDone {
    /// Base64 of the WAV file.
    pub wav: String,
}

impl Incoming {
    pub(crate) fn parse(line: &[u8]) -> Result<Incoming, String> {
        let mut message: Map<String, Value> =
            serde_json::from_slice(line).map_err(|error| error.to_string())?;
        if message.get("type").and_then(Value::as_str) == Some("done") {
            message.remove("type");
            let id = message
                .remove("id")
                .and_then(|id| id.as_u64())
                .ok_or("done without an id")?;
            return Ok(Incoming::Done {
                id,
                fields: message,
            });
        }
        let fixed: Fixed =
            serde_json::from_value(Value::Object(message)).map_err(|error| error.to_string())?;
        Ok(match fixed {
            Fixed::Ready { versions } => Incoming::Ready { versions },
            Fixed::Progress { id, tokens } => Incoming::Progress { id, tokens },
            Fixed::Cancelled { id } => Incoming::Cancelled { id },
            Fixed::Error { id, message } => Incoming::Error { id, message },
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn parse(value: Value) -> Result<Incoming, String> {
        Incoming::parse(value.to_string().as_bytes())
    }

    #[test]
    fn reads_each_type() {
        let ready = parse(json!({"type": "ready", "versions": {"python": "3.12.3"}})).unwrap();
        assert_eq!(
            ready,
            Incoming::Ready {
                versions: [("python".into(), "3.12.3".into())].into()
            }
        );
        assert_eq!(
            parse(json!({"type": "progress", "id": 1, "tokens": 64})),
            Ok(Incoming::Progress { id: 1, tokens: 64 })
        );
        assert_eq!(
            parse(json!({"type": "cancelled", "id": 2})),
            Ok(Incoming::Cancelled { id: 2 })
        );
        assert_eq!(
            parse(json!({"type": "error", "id": null, "message": "model path not found: /m"})),
            Ok(Incoming::Error {
                id: None,
                message: "model path not found: /m".into()
            })
        );
        assert_eq!(
            parse(json!({"type": "error", "id": 3, "message": "x"})),
            Ok(Incoming::Error {
                id: Some(3),
                message: "x".into()
            })
        );
    }

    #[test]
    fn done_keeps_its_fields_for_the_call() {
        let done = parse(
            json!({"type": "done", "id": 1, "text": "やあ", "tokens": 2, "finish": "length"}),
        );
        let Ok(Incoming::Done { id: 1, fields }) = done else {
            panic!("{done:?}")
        };
        let llm: LlmDone = serde_json::from_value(Value::Object(fields)).unwrap();
        assert_eq!(
            llm,
            LlmDone {
                text: "やあ".into(),
                tokens: 2,
                finish: LlmFinish::Length
            }
        );
        let tts = json!({"wav": "UklGRg=="});
        assert!(serde_json::from_value::<TtsDone>(tts).is_ok());
        assert!(serde_json::from_value::<TtsDone>(json!({"wav": "", "rate": 24000})).is_err());
        assert!(
            serde_json::from_value::<LlmDone>(json!({"text": "", "tokens": 1, "finish": "abort"}))
                .is_err()
        );
    }

    #[test]
    fn rejects_what_neither_side_knows() {
        assert!(parse(json!({"type": "progress", "id": 1, "tokens": 64, "extra": 1})).is_err());
        assert!(parse(json!({"type": "hello"})).is_err());
        assert!(parse(json!({"type": "progress", "id": 1})).is_err());
        assert!(parse(json!({"type": "done", "text": ""})).is_err());
        assert!(Incoming::parse(b"flash-attn is not installed").is_err());
    }
}
