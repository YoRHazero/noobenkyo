//! What `llm_chat` gives back. The request body is llama-server's own JSON,
//! put together by the caller.

use serde::Deserialize;

/// Tokens written so far, reported every 64.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LlmProgress {
    pub tokens: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LlmReply {
    pub text: String,
    pub tokens: u32,
    pub finish: LlmFinish,
    /// Where it ran, for the user to see.
    pub machine: String,
}

/// Whether the model stopped on its own or ran into `max_tokens`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LlmFinish {
    Stop,
    Length,
}
