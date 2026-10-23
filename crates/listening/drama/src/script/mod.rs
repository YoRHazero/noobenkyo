//! Scripts in the drama: storing a script as an episode with each speaker
//! mapped to a voice, appending to an episode, and the context a script is
//! written from. Parsing the script JSON lives in listening-script.

pub mod rules;
pub mod store;

mod values;

pub use values::{MainCharacter, Script, ScriptContext, ScriptEpisode, ScriptLine};
