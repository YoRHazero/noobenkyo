use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
};

use crate::by_key;
use crate::entity::reference;
use crate::{CharacterDetail, Gender, Language, ReferenceText};

/// One character with the texts of its references; `None` if it is gone.
pub async fn character(
    db: &impl ConnectionTrait,
    name: &str,
) -> Result<Option<CharacterDetail>, DbErr> {
    let Some(row) = by_key::find_character(db, name).await? else {
        return Ok(None);
    };
    let texts: Vec<(String, String)> = reference::Entity::find()
        .select_only()
        .column(reference::Column::Language)
        .column(reference::Column::Text)
        .filter(reference::Column::CharacterId.eq(row.id))
        .order_by_asc(reference::Column::Language)
        .into_tuple()
        .all(db)
        .await?;
    let references = texts
        .into_iter()
        .map(|(code, text)| {
            Ok(ReferenceText {
                language: Language::from_code(&code)?,
                text,
            })
        })
        .collect::<Result<_, DbErr>>()?;
    Ok(Some(CharacterDetail {
        gender: Gender::from_code(&row.gender)?,
        age: u32::try_from(row.age).unwrap_or(0),
        name: row.name,
        kana: row.kana,
        prompt: row.prompt,
        references,
    }))
}
