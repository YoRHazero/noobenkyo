use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    Set,
};

use super::ID_CHUNK;
use crate::entity::word;

pub async fn find_word(
    db: &impl ConnectionTrait,
    writing: &str,
    origin: &str,
) -> Result<Option<word::Model>, DbErr> {
    word::Entity::find()
        .filter(word::Column::Writing.eq(writing))
        .filter(word::Column::Origin.eq(origin))
        .one(db)
        .await
}

pub async fn insert_word(
    db: &impl ConnectionTrait,
    writing: &str,
    origin: &str,
    origin_language: &str,
) -> Result<word::Model, DbErr> {
    word::ActiveModel {
        writing: Set(writing.to_owned()),
        origin: Set(origin.to_owned()),
        origin_language: Set(origin_language.to_owned()),
        ..Default::default()
    }
    .insert(db)
    .await
}

pub async fn set_origin_language(
    db: &impl ConnectionTrait,
    id: i64,
    origin_language: &str,
) -> Result<(), DbErr> {
    word::ActiveModel {
        id: Set(id),
        origin_language: Set(origin_language.to_owned()),
        ..Default::default()
    }
    .update(db)
    .await
    .map(|_| ())
}

/// Words with the given ids, in id order.
pub async fn words_by_ids(
    db: &impl ConnectionTrait,
    ids: &[i64],
) -> Result<Vec<word::Model>, DbErr> {
    let mut found = Vec::with_capacity(ids.len());
    for chunk in ids.chunks(ID_CHUNK) {
        let rows = word::Entity::find()
            .filter(word::Column::Id.is_in(chunk.iter().copied()))
            .order_by_asc(word::Column::Id)
            .all(db)
            .await?;
        found.extend(rows);
    }
    Ok(found)
}

/// Words written with any of the given writings, whatever their origin.
pub async fn words_with_writings(
    db: &impl ConnectionTrait,
    writings: &[&str],
) -> Result<Vec<word::Model>, DbErr> {
    let mut found = Vec::new();
    for chunk in writings.chunks(ID_CHUNK) {
        let rows = word::Entity::find()
            .filter(word::Column::Writing.is_in(chunk.iter().copied()))
            .all(db)
            .await?;
        found.extend(rows);
    }
    Ok(found)
}

/// Deletes a word; its readings, senses, labels and sentences go with it.
pub async fn delete_word(db: &impl ConnectionTrait, id: i64) -> Result<(), DbErr> {
    word::Entity::delete_by_id(id).exec(db).await.map(|_| ())
}
