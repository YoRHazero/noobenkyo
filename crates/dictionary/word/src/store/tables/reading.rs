use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    Set,
};

use super::ID_CHUNK;
use crate::entity::reading;

pub async fn find_reading(
    db: &impl ConnectionTrait,
    word_id: i64,
    kana: &str,
) -> Result<Option<reading::Model>, DbErr> {
    reading::Entity::find()
        .filter(reading::Column::WordId.eq(word_id))
        .filter(reading::Column::Kana.eq(kana))
        .one(db)
        .await
}

pub async fn insert_reading(
    db: &impl ConnectionTrait,
    word_id: i64,
    kana: &str,
) -> Result<reading::Model, DbErr> {
    reading::ActiveModel {
        word_id: Set(word_id),
        kana: Set(kana.to_owned()),
        ..Default::default()
    }
    .insert(db)
    .await
}

pub async fn readings_of_word(
    db: &impl ConnectionTrait,
    word_id: i64,
) -> Result<Vec<reading::Model>, DbErr> {
    reading::Entity::find()
        .filter(reading::Column::WordId.eq(word_id))
        .order_by_asc(reading::Column::Id)
        .all(db)
        .await
}

/// Readings with the given ids, in id order.
pub async fn readings_by_ids(
    db: &impl ConnectionTrait,
    ids: &[i64],
) -> Result<Vec<reading::Model>, DbErr> {
    let mut found = Vec::with_capacity(ids.len());
    for chunk in ids.chunks(ID_CHUNK) {
        let rows = reading::Entity::find()
            .filter(reading::Column::Id.is_in(chunk.iter().copied()))
            .order_by_asc(reading::Column::Id)
            .all(db)
            .await?;
        found.extend(rows);
    }
    Ok(found)
}

/// Deletes a reading; its senses, labels and sentences go with it.
pub async fn delete_reading(db: &impl ConnectionTrait, id: i64) -> Result<(), DbErr> {
    reading::Entity::delete_by_id(id).exec(db).await.map(|_| ())
}
