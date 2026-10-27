use serde::Serialize;

use crate::Gender;

/// A script that passed the check: what the standard JSON carries.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct Script {
    pub title: String,
    pub summary: String,
    pub lines: Vec<ScriptLine>,
}

/// One line of a script, before its speaker is mapped to a voice.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct ScriptLine {
    pub speaker: String,
    /// May carry reading marks `{writing|reading}`.
    pub ja: String,
    pub zh: String,
}

/// What a script is written from. How much of it reaches the model is up to
/// the writer: an agent, or the ai group.
#[derive(Clone, Debug, PartialEq, Serialize, specta::Type)]
pub struct ScriptContext {
    pub world: String,
    pub setting: String,
    pub mains: Vec<MainCharacter>,
    /// In the order the user chose.
    pub predecessors: Vec<ScriptEpisode>,
    /// The episode being continued, if any.
    pub appending: Option<ScriptEpisode>,
}

/// A main character as the model sees it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct MainCharacter {
    pub name: String,
    pub kana: String,
    pub gender: Gender,
    pub age: u32,
    pub description: String,
}

/// An episode's content without audio and voices.
#[derive(Clone, Debug, PartialEq, Serialize, specta::Type)]
pub struct ScriptEpisode {
    pub number: f64,
    pub title: String,
    pub summary: String,
    pub lines: Vec<ScriptLine>,
}
