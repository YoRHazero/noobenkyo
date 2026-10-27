//! One command per function of listening the frontend uses, handed on as
//! is, in files named after listening's own. Audio crosses as plain bytes in
//! JSON, like every other value. Writing scripts and synthesizing audio go
//! through the agent and ai groups, not through these.

pub mod characters;
pub mod episodes;
pub mod lines;
pub mod script;
pub mod worlds;
