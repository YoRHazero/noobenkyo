//! The listening group's public face: `ListeningService`, the pure function
//! `check_script`, and the sub-crate types that leave the group. Only
//! deterministic rules and storage live here; writing scripts, premises and
//! synthesizing audio go through the ai group, which calls these functions.

mod characters;
mod clock;
mod connection;
mod episodes;
mod error;
mod lines;
mod script;
mod service;
mod worlds;

pub use error::ListeningError;
pub use script::check_script;
pub use service::ListeningService;

pub use listening_drama::{
    CastMember, CharacterDetail, CharacterSummary, EpisodeDetail, EpisodeSummary, Gender, Language,
    LineId, LineView, MainCharacter, NewCharacter, NewReference, Reference, ReferenceText, Role,
    ScriptContext, ScriptEpisode, ScriptLine, SpeechItem, WorldDetail, WorldSummary,
};
pub use uuid::Uuid;
