//! Adding a missing reference recording, and reading one.

use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Set, TransactionTrait,
};

use crate::by_key;
use crate::character::rules::references::reference_problems;
use crate::entity::reference;
use crate::{DramaError, Language, Reference};

/// Adds the reference recording a character lacks in `language`. A
/// recording, once there, is never replaced.
pub async fn add_reference(
    db: &DatabaseConnection,
    name: &str,
    language: Language,
    audio: &[u8],
    text: &str,
) -> Result<(), DramaError> {
    let problems = reference_problems(language, audio, text);
    if !problems.is_empty() {
        return Err(DramaError::BadReference(problems.join("；")));
    }
    let txn = db.begin().await?;
    let found = by_key::require_character(&txn, name).await?;
    let existing = reference::Entity::find_by_id((found.id, language.code().to_owned()))
        .one(&txn)
        .await?;
    if existing.is_some() {
        return Err(DramaError::ReferenceExists {
            character: found.name,
            language: language.name(),
        });
    }
    reference::ActiveModel {
        character_id: Set(found.id),
        language: Set(language.code().to_owned()),
        audio: Set(audio.to_vec()),
        text: Set(text.trim().to_owned()),
    }
    .insert(&txn)
    .await?;
    txn.commit().await?;
    Ok(())
}

/// A character's reference recording in `language`; `None` if it has none.
pub async fn reference(
    db: &impl ConnectionTrait,
    name: &str,
    language: Language,
) -> Result<Option<Reference>, DramaError> {
    let found = by_key::require_character(db, name).await?;
    let row = reference::Entity::find_by_id((found.id, language.code().to_owned()))
        .one(db)
        .await?;
    Ok(row.map(|row| Reference {
        audio: row.audio,
        text: row.text,
    }))
}
