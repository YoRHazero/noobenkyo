use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    Set,
};

use crate::entity::sentence;

pub async fn sentences_of_sense(
    db: &impl ConnectionTrait,
    sense_id: i64,
) -> Result<Vec<sentence::Model>, DbErr> {
    sentence::Entity::find()
        .filter(sentence::Column::SenseId.eq(sense_id))
        .order_by_asc(sentence::Column::Id)
        .all(db)
        .await
}

pub async fn insert_sentence(
    db: &impl ConnectionTrait,
    sense_id: i64,
    marked: &str,
    translation: Option<&str>,
) -> Result<sentence::Model, DbErr> {
    sentence::ActiveModel {
        sense_id: Set(sense_id),
        marked: Set(marked.to_owned()),
        translation: Set(translation.map(str::to_owned)),
        ..Default::default()
    }
    .insert(db)
    .await
}

pub async fn delete_sentence(db: &impl ConnectionTrait, id: i64) -> Result<(), DbErr> {
    sentence::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map(|_| ())
}

/// All sentences, grouped by sense in creation order.
pub async fn all_sentences(db: &impl ConnectionTrait) -> Result<Vec<sentence::Model>, DbErr> {
    sentence::Entity::find()
        .order_by_asc(sentence::Column::SenseId)
        .order_by_asc(sentence::Column::Id)
        .all(db)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::test_db::{memory_db, sense_of};

    #[tokio::test]
    async fn inserts_lists_and_deletes() {
        let db = memory_db().await;
        let sense = sense_of(&db, "明日", "あした", "明天").await;
        let first = insert_sentence(&db, sense, "[[{明日|あした}]]", Some("明天"))
            .await
            .unwrap();
        insert_sentence(&db, sense, "[[{明日|あした}]]だ", None)
            .await
            .unwrap();
        assert_eq!(sentences_of_sense(&db, sense).await.unwrap().len(), 2);
        delete_sentence(&db, first.id).await.unwrap();
        let rest = all_sentences(&db).await.unwrap();
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0].translation, None);
    }
}
