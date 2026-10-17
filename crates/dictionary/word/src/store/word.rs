use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    Set,
};

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
    stratum: &str,
) -> Result<word::Model, DbErr> {
    word::ActiveModel {
        writing: Set(writing.to_owned()),
        origin: Set(origin.to_owned()),
        stratum: Set(stratum.to_owned()),
        ..Default::default()
    }
    .insert(db)
    .await
}

pub async fn set_stratum(db: &impl ConnectionTrait, id: i64, stratum: &str) -> Result<(), DbErr> {
    word::ActiveModel {
        id: Set(id),
        stratum: Set(stratum.to_owned()),
        ..Default::default()
    }
    .update(db)
    .await
    .map(|_| ())
}

/// All words in creation order.
pub async fn all_words(db: &impl ConnectionTrait) -> Result<Vec<word::Model>, DbErr> {
    word::Entity::find()
        .order_by_asc(word::Column::Id)
        .all(db)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::test_db::memory_db;

    #[tokio::test]
    async fn inserts_and_finds_by_writing_and_origin() {
        let db = memory_db().await;
        let coat = insert_word(&db, "コート", "coat", "").await.unwrap();
        insert_word(&db, "コート", "court", "").await.unwrap();
        let found = find_word(&db, "コート", "coat").await.unwrap().unwrap();
        assert_eq!(found.id, coat.id);
        assert!(find_word(&db, "コート", "").await.unwrap().is_none());
    }

    #[tokio::test]
    async fn rejects_duplicate_key() {
        let db = memory_db().await;
        insert_word(&db, "明日", "", "").await.unwrap();
        assert!(insert_word(&db, "明日", "", "").await.is_err());
    }

    #[tokio::test]
    async fn sets_stratum() {
        let db = memory_db().await;
        let word = insert_word(&db, "明日", "", "").await.unwrap();
        set_stratum(&db, word.id, "和语").await.unwrap();
        assert_eq!(
            find_word(&db, "明日", "").await.unwrap().unwrap().stratum,
            "和语"
        );
    }

    #[tokio::test]
    async fn lists_all_in_creation_order() {
        let db = memory_db().await;
        insert_word(&db, "b", "", "").await.unwrap();
        insert_word(&db, "a", "", "").await.unwrap();
        let writings: Vec<_> = all_words(&db)
            .await
            .unwrap()
            .into_iter()
            .map(|w| w.writing)
            .collect();
        assert_eq!(writings, ["b", "a"]);
    }
}
