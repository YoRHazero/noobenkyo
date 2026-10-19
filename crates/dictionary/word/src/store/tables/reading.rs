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

/// All readings in creation order.
#[cfg(test)]
pub async fn all_readings(db: &impl ConnectionTrait) -> Result<Vec<reading::Model>, DbErr> {
    reading::Entity::find()
        .order_by_asc(reading::Column::Id)
        .all(db)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::tables::word::insert_word;
    use crate::store::test_db::memory_db;

    #[tokio::test]
    async fn inserts_finds_and_lists() {
        let db = memory_db().await;
        let word = insert_word(&db, "明日", "", "").await.unwrap();
        let ashita = insert_reading(&db, word.id, "あした").await.unwrap();
        insert_reading(&db, word.id, "あす").await.unwrap();
        assert_eq!(
            find_reading(&db, word.id, "あした")
                .await
                .unwrap()
                .unwrap()
                .id,
            ashita.id
        );
        assert!(
            find_reading(&db, word.id, "みょうにち")
                .await
                .unwrap()
                .is_none()
        );
        assert_eq!(all_readings(&db).await.unwrap().len(), 2);
        assert!(insert_reading(&db, word.id, "あす").await.is_err());
    }
}
