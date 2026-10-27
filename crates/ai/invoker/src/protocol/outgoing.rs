use serde::Serialize;
use serde_json::{Map, Value};

use crate::TtsLanguage;

/// What `start` tells an executor to load, with its runtime's own fields.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "runtime", rename_all = "snake_case")]
pub(crate) enum StartRuntime {
    LlamaCpp {
        server: String,
        path: String,
        args: Vec<String>,
    },
    QwenTts {
        path: String,
    },
}

/// A call as the executor takes it, short of the id each executor numbers
/// on its own.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Request {
    LlmChat {
        body: Map<String, Value>,
    },
    TtsClone {
        text: String,
        language: TtsLanguage,
        /// Base64 of the WAV file.
        ref_audio: String,
        ref_text: String,
    },
    TtsDesign {
        text: String,
        language: TtsLanguage,
        instruct: String,
        seed: Option<u64>,
    },
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Outgoing<'a> {
    Start {
        #[serde(flatten)]
        runtime: &'a StartRuntime,
        heartbeat_timeout_secs: u64,
    },
    LlmChat {
        id: u64,
        body: &'a Map<String, Value>,
    },
    TtsClone {
        id: u64,
        text: &'a str,
        language: TtsLanguage,
        ref_audio: &'a str,
        ref_text: &'a str,
    },
    TtsDesign {
        id: u64,
        text: &'a str,
        language: TtsLanguage,
        instruct: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        seed: Option<u64>,
    },
    Cancel {
        id: u64,
    },
    Ping,
}

fn line(message: &Outgoing) -> String {
    serde_json::to_string(message).expect("messages always serialize")
}

pub(crate) fn start_line(runtime: &StartRuntime, heartbeat_timeout_secs: u64) -> String {
    line(&Outgoing::Start {
        runtime,
        heartbeat_timeout_secs,
    })
}

pub(crate) fn request_line(id: u64, request: &Request) -> String {
    line(&match request {
        Request::LlmChat { body } => Outgoing::LlmChat { id, body },
        Request::TtsClone {
            text,
            language,
            ref_audio,
            ref_text,
        } => Outgoing::TtsClone {
            id,
            text,
            language: *language,
            ref_audio,
            ref_text,
        },
        Request::TtsDesign {
            text,
            language,
            instruct,
            seed,
        } => Outgoing::TtsDesign {
            id,
            text,
            language: *language,
            instruct,
            seed: *seed,
        },
    })
}

pub(crate) fn cancel_line(id: u64) -> String {
    line(&Outgoing::Cancel { id })
}

pub(crate) fn ping_line() -> String {
    line(&Outgoing::Ping)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn parsed(line: String) -> Value {
        assert!(!line.contains('\n'));
        serde_json::from_str(&line).unwrap()
    }

    #[test]
    fn start_carries_the_runtime_fields_at_the_top_level() {
        let llama = StartRuntime::LlamaCpp {
            server: "/opt/llama-server".into(),
            path: "/models/a b.gguf".into(),
            args: vec!["-c".into(), "16384".into()],
        };
        assert_eq!(
            parsed(start_line(&llama, 30)),
            json!({"type": "start", "runtime": "llama_cpp", "server": "/opt/llama-server",
                   "path": "/models/a b.gguf", "args": ["-c", "16384"], "heartbeat_timeout_secs": 30})
        );
        let tts = StartRuntime::QwenTts {
            path: "/models/base".into(),
        };
        assert_eq!(
            parsed(start_line(&tts, 30)),
            json!({"type": "start", "runtime": "qwen_tts", "path": "/models/base", "heartbeat_timeout_secs": 30})
        );
    }

    #[test]
    fn requests_take_their_id() {
        let body =
            json!({"messages": [{"role": "user", "content": "一行目\n二行目"}], "max_tokens": 8});
        let chat = Request::LlmChat {
            body: body.as_object().unwrap().clone(),
        };
        assert_eq!(
            parsed(request_line(1, &chat)),
            json!({"type": "llm_chat", "id": 1, "body": body})
        );
        let clone = Request::TtsClone {
            text: "おはよう".into(),
            language: TtsLanguage::Japanese,
            ref_audio: "UklGRg==".into(),
            ref_text: "はじめまして".into(),
        };
        assert_eq!(
            parsed(request_line(2, &clone)),
            json!({"type": "tts_clone", "id": 2, "text": "おはよう", "language": "Japanese",
                   "ref_audio": "UklGRg==", "ref_text": "はじめまして"})
        );
    }

    #[test]
    fn a_design_without_a_seed_leaves_it_out() {
        let design = |seed| Request::TtsDesign {
            text: "你好".into(),
            language: TtsLanguage::Chinese,
            instruct: "温柔的女声".into(),
            seed,
        };
        assert_eq!(
            parsed(request_line(3, &design(None))),
            json!({"type": "tts_design", "id": 3, "text": "你好", "language": "Chinese", "instruct": "温柔的女声"})
        );
        assert_eq!(parsed(request_line(4, &design(Some(7))))["seed"], 7);
    }

    #[test]
    fn controls() {
        assert_eq!(parsed(cancel_line(5)), json!({"type": "cancel", "id": 5}));
        assert_eq!(parsed(ping_line()), json!({"type": "ping"}));
    }
}
