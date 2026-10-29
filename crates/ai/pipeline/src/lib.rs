//! What the app does with models, one kind of job at a time: builds each
//! call for the model the caller names, runs the loop. Speaks lines and
//! designs voices for now; writing scripts comes later. Uses its own types and touches no
//! storage: core converts the features' data into them and stores what comes
//! back. See dev_docs/Pipeline.md.

mod design;
mod error;
mod language;
mod speak;
mod voice_sample;

pub use design::design_voice;
pub use error::PipelineError;
pub use language::Language;
pub use speak::{SpeakItem, Spoken, speak_lines};
pub use voice_sample::VoiceSample;
