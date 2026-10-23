//! The listening drama itself: characters, worlds and their casts, episodes
//! and their lines with audio. The schema, the generated entities, the rules,
//! and every read and write of the tables, one folder per unit.

#[rustfmt::skip]
pub mod entity;
mod migration;

pub mod character;
pub mod episode;
pub mod line;
pub mod script;
pub mod text;
pub mod world;

mod by_key;
mod error;
mod language;

pub use character::{
    CharacterDetail, CharacterSummary, Gender, NewCharacter, NewReference, Reference, ReferenceText,
};
pub use episode::{EpisodeDetail, EpisodeSummary};
pub use error::DramaError;
pub use language::Language;
pub use line::{LineId, LineView, SpeechItem};
pub use migration::Migrator;
pub use script::{MainCharacter, Script, ScriptContext, ScriptEpisode, ScriptLine};
pub use world::{CastMember, Role, WorldDetail, WorldSummary};
