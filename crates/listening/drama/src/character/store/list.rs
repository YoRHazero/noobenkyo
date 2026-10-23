use sea_orm::{ConnectionTrait, DbErr, EntityTrait, QueryOrder};

use super::languages::languages_by_id;
use crate::entity::character;
use crate::{CharacterSummary, Gender};

/// Every character, in the order they were created.
pub async fn characters(db: &impl ConnectionTrait) -> Result<Vec<CharacterSummary>, DbErr> {
    let rows = character::Entity::find()
        .order_by_asc(character::Column::Id)
        .all(db)
        .await?;
    let mut languages = languages_by_id(db, None).await?;
    rows.into_iter()
        .map(|row| {
            Ok(CharacterSummary {
                languages: languages.remove(&row.id).unwrap_or_default(),
                has_prompt: row.prompt.is_some(),
                gender: Gender::from_code(&row.gender)?,
                age: u32::try_from(row.age).unwrap_or(0),
                name: row.name,
                kana: row.kana,
            })
        })
        .collect()
}
