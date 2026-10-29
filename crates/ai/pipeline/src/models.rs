//! The model each job runs on. The names are an agreement with the
//! settings: a machine offers a model under the same name.

/// Speaks a line in the voice of a reference recording.
pub(crate) const SPEAK: &str = "qwen3-tts-1.7b-base";

/// Makes up a voice from a description.
pub(crate) const DESIGN_VOICE: &str = "qwen3-tts-1.7b-voicedesign";
