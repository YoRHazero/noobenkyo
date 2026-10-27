//! The TTS calls' requests and reply. qwen-tts has no HTTP API to borrow, so
//! these fields are ours.

use serde::Serialize;

/// Clones the voice of a reference recording.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TtsCloneRequest {
    pub text: String,
    pub language: TtsLanguage,
    /// A WAV file's bytes.
    pub ref_audio: Vec<u8>,
    /// What the reference recording says.
    pub ref_text: String,
}

/// Makes up a voice from a description.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TtsDesignRequest {
    pub text: String,
    pub language: TtsLanguage,
    pub instruct: String,
    pub seed: Option<u64>,
}

/// The languages the model knows, by the names it uses; not listening's Language.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum TtsLanguage {
    Japanese,
    Chinese,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TtsReply {
    /// Mono 16-bit WAV at the model's own sample rate.
    pub wav: Vec<u8>,
    /// Where it ran, for the user to see.
    pub machine: String,
}
