use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    Set,
};

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

/// All readings in creation order.
pub async fn all_readings(db: &impl ConnectionTrait) -> Result<Vec<reading::Model>, DbErr> {
    reading::Entity::find()
        .order_by_asc(reading::Column::Id)
        .all(db)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::test_db::memory_db;
    use crate::store::word::insert_word;

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
