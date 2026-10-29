//! Helpers for the behaviour tests.
//!
//! The GPU tests take the machines from the same file as ai-invoker's: the
//! `InvokerSettings` JSON that `NOOBENKYO_INVOKER_TEST_SETTINGS` points at.
//! Its machines must offer the two models named below. Run them one at a
//! time:
//!
//! NOOBENKYO_INVOKER_TEST_SETTINGS=<file> cargo test -p ai-pipeline -- --ignored --test-threads=1

use ai_invoker::{Invoker, InvokerSettings};
use ai_pipeline::{Language, SpeakItem, VoiceSample};

const SETTINGS_VARIABLE: &str = "NOOBENKYO_INVOKER_TEST_SETTINGS";
pub const SPEAK: &str = "qwen3-tts-1.7b-base";
pub const DESIGN: &str = "qwen3-tts-1.7b-voicedesign";

pub fn gpu_invoker() -> Invoker {
    let path = std::env::var(SETTINGS_VARIABLE)
        .unwrap_or_else(|_| panic!("{SETTINGS_VARIABLE} is not set"));
    let text = std::fs::read_to_string(&path).unwrap();
    let settings: InvokerSettings = serde_json::from_str(&text).unwrap();
    settings.validate().unwrap();
    Invoker::new(settings)
}

pub fn item(voice: &str, language: Language, text: &str) -> SpeakItem {
    SpeakItem {
        text: text.into(),
        language,
        voice: voice.into(),
    }
}

pub fn sample(voice: &str, language: Language, audio: Vec<u8>, text: &str) -> VoiceSample {
    VoiceSample {
        voice: voice.into(),
        language,
        audio,
        text: text.into(),
    }
}

pub fn is_wav(bytes: &[u8]) -> bool {
    bytes.len() > 44 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WAVE"
}
