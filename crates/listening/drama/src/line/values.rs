use sea_orm::DbErr;
use serde::{Deserialize, Serialize};

use crate::{Language, RubySegment};

/// Identifies a line. Valid on this machine only. A `u32`, so it reaches the
/// frontend as a plain number; the table's row id is 64-bit.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, specta::Type,
)]
#[serde(transparent)]
pub struct LineId(pub u32);

impl LineId {
    /// How the id is written in the tables.
    pub(crate) fn row(self) -> i64 {
        i64::from(self.0)
    }

    pub(crate) fn from_row(id: i64) -> Result<Self, DbErr> {
        u32::try_from(id)
            .map(LineId)
            .map_err(|_| DbErr::Type(format!("line id {id} is out of range")))
    }
}

/// A line of an episode as shown to the user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct LineView {
    pub id: LineId,
    /// The name written in the script.
    pub speaker: String,
    /// The character whose voice reads it.
    pub voice: String,
    /// May carry reading marks `{writing|reading}`; this is what editing starts from.
    pub ja: String,
    /// `ja` split into text and ruby, for showing it.
    pub ja_ruby: Vec<RubySegment>,
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
