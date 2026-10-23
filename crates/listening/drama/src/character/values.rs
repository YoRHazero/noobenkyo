use sea_orm::DbErr;
use serde::{Deserialize, Serialize};

use crate::Language;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Female,
    Male,
}

impl Gender {
    /// How the gender is written in the tables.
    pub(crate) fn code(self) -> &'static str {
        match self {
            Gender::Female => "female",
            Gender::Male => "male",
        }
    }

    pub(crate) fn from_code(code: &str) -> Result<Self, DbErr> {
        match code {
            "female" => Ok(Gender::Female),
            "male" => Ok(Gender::Male),
            _ => Err(DbErr::Type(format!("unknown gender {code}"))),
        }
    }
}

/// A character to create.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct NewCharacter {
    pub name: String,
    pub kana: String,
    pub gender: Gender,
    pub age: u32,
    /// The full description handed to VoiceDesign, if the references were made with it.
    pub prompt: Option<String>,
    pub references: Vec<NewReference>,
}

/// A reference recording and the text spoken in it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct NewReference {
    pub language: Language,
    pub audio: Vec<u8>,
    pub text: String,
}

/// A character in the list.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct CharacterSummary {
    pub name: String,
    pub kana: String,
    pub gender: Gender,
    pub age: u32,
    /// Languages it has a reference recording in.
    pub languages: Vec<Language>,
    pub has_prompt: bool,
}

/// One character in full, without the audio.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct CharacterDetail {
    pub name: String,
    pub kana: String,
    pub gender: Gender,
    pub age: u32,
    pub prompt: Option<String>,
    pub references: Vec<ReferenceText>,
}

/// The text of one reference recording.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct ReferenceText {
    pub language: Language,
    pub text: String,
}

/// A reference recording with its text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct Reference {
    pub audio: Vec<u8>,
    pub text: String,
}
