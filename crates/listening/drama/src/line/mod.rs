//! Lines of an episode and their audio: who speaks, the Japanese and Chinese
//! text, and a recording of each.

pub mod rules;
pub mod store;

mod values;

pub use values::{LineId, LineView, SpeechItem};
