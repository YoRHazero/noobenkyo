//! Which languages characters have reference recordings in, read without
//! the audio.

use std::collections::HashMap;

use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, JoinType, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait,
};

use crate::Language;
use crate::entity::{character, reference};

/// Reference languages of the given characters (all of them when `None`),
/// by character id, in language order.
pub async fn languages_by_id(
    db: &impl ConnectionTrait,
    character_ids: Option<&[i64]>,
) -> Result<HashMap<i64, Vec<Language>>, DbErr> {
    let mut query = reference::Entity::find()
        .select_only()
        .column(reference::Column::CharacterId)
        .column(reference::Column::Language)
        .order_by_asc(reference::Column::Language);
    if let Some(ids) = character_ids {
        query = query.filter(reference::Column::CharacterId.is_in(ids.iter().copied()));
    }
    let rows: Vec<(i64, String)> = query.into_tuple().all(db).await?;
    let mut found: HashMap<i64, Vec<Language>> = HashMap::new();
    for (id, code) in rows {
        found
            .entry(id)
            .or_default()
            .push(Language::from_code(&code)?);
    }
    Ok(found)
}

/// Reference languages of the characters with the given names, by name.
/// Names of characters that are gone are left out.
pub async fn languages_by_name(
    db: &impl ConnectionTrait,
    names: &[String],
) -> Result<HashMap<String, Vec<Language>>, DbErr> {
    let rows: Vec<(String, String)> = reference::Entity::find()
        .select_only()
        .column(character::Column::Name)
        .column(reference::Column::Language)
        .join(JoinType::InnerJoin, reference::Relation::Character.def())
        .filter(character::Column::Name.is_in(names.iter().cloned()))
        .order_by_asc(reference::Column::Language)
        .into_tuple()
        .all(db)
        .await?;
    let mut found: HashMap<String, Vec<Language>> = HashMap::new();
    for (name, code) in rows {
        found
            .entry(name)
            .or_default()
            .push(Language::from_code(&code)?);
    }
    Ok(found)
}
