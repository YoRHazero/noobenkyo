use sea_orm::DbErr;
use serde::{Deserialize, Serialize};

use crate::Language;

/// What a character is in a world. Only main characters have a description:
/// personality, occupation, history, abilities and relationships.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Role {
    Narrator,
    Main { description: String },
    Minor,
}

impl Role {
    /// How the role is written in the role and description columns.
    pub(crate) fn columns(&self) -> (&'static str, Option<String>) {
        match self {
            Role::Narrator => ("narrator", None),
            Role::Main { description } => ("main", Some(description.clone())),
            Role::Minor => ("minor", None),
        }
    }

    pub(crate) fn from_columns(role: &str, description: Option<String>) -> Result<Self, DbErr> {
        match (role, description) {
            ("narrator", None) => Ok(Role::Narrator),
            ("main", Some(description)) => Ok(Role::Main { description }),
            ("minor", None) => Ok(Role::Minor),
            (role, description) => Err(DbErr::Type(format!(
                "invalid role {role} with description {description:?}"
            ))),
        }
    }
}

/// A world in the list.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct WorldSummary {
    pub name: String,
    pub episodes: u32,
}

/// One world with its cast.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct WorldDetail {
    pub name: String,
    pub setting: String,
    /// The narrator first, then main characters, then minor ones.
    pub cast: Vec<CastMember>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct CastMember {
    pub character: String,
    pub role: Role,
    /// Languages the character has a reference recording in.
    pub languages: Vec<Language>,
}
