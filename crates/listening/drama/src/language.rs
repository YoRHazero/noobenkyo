use sea_orm::DbErr;
use serde::{Deserialize, Serialize};

/// The language of a reference recording, a line's text, or its audio.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, specta::Type,
)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Ja,
    Zh,
}

impl Language {
    pub const ALL: [Language; 2] = [Language::Ja, Language::Zh];

    /// The name used in messages.
    pub fn name(self) -> &'static str {
        match self {
            Language::Ja => "日语",
            Language::Zh => "中文",
        }
    }

    /// How the language is written in the tables.
    pub(crate) fn code(self) -> &'static str {
        match self {
            Language::Ja => "ja",
            Language::Zh => "zh",
        }
    }

    pub(crate) fn from_code(code: &str) -> Result<Self, DbErr> {
        match code {
            "ja" => Ok(Language::Ja),
            "zh" => Ok(Language::Zh),
            _ => Err(DbErr::Type(format!("unknown language {code}"))),
        }
    }
}
