use serde::{Deserialize, Serialize};

use crate::Language;

/// Identifies a line. Valid on this machine only.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, specta::Type,
)]
#[serde(transparent)]
pub struct LineId(pub i64);

/// A line of an episode as shown to the user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct LineView {
    pub id: LineId,
    /// The name written in the script.
    pub speaker: String,
    /// The character whose voice reads it.
    pub voice: String,
    /// May carry reading marks `{writing|reading}`.
    pub ja: String,
    pub zh: String,
    /// Languages it has audio in.
    pub audio: Vec<Language>,
}

/// A missing recording that can be synthesized now.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct SpeechItem {
    pub line: LineId,
    pub language: Language,
    /// The text to speak, with reading marks replaced by their readings.
    pub text: String,
    /// The character whose reference recording in `language` to clone.
    pub voice: String,
}
