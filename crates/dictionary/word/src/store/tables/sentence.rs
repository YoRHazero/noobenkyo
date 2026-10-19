use std::collections::HashMap;

use sea_orm::sea_query::{Expr, ExprTrait};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};

use super::ID_CHUNK;
use crate::SentenceContent;
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

/// Sentences of the given senses, by sense and then creation order.
pub async fn sentences_of_senses(
    db: &impl ConnectionTrait,
    sense_ids: &[i64],
) -> Result<Vec<sentence::Model>, DbErr> {
    let mut found = Vec::new();
    for chunk in sense_ids.chunks(ID_CHUNK) {
        let rows = sentence::Entity::find()
            .filter(sentence::Column::SenseId.is_in(chunk.iter().copied()))
            .order_by_asc(sentence::Column::SenseId)
            .order_by_asc(sentence::Column::Id)
            .all(db)
            .await?;
        found.extend(rows);
    }
    Ok(found)
}

/// Number of sentences of each given sense; senses without sentences are absent.
pub async fn sentence_counts(
    db: &impl ConnectionTrait,
    sense_ids: &[i64],
) -> Result<HashMap<i64, u32>, DbErr> {
    let mut counts = HashMap::new();
    for chunk in sense_ids.chunks(ID_CHUNK) {
        let rows: Vec<(i64, i64)> = sentence::Entity::find()
            .select_only()
            .column(sentence::Column::SenseId)
            .column_as(Expr::col(sentence::Column::Id).count(), "count")
            .filter(sentence::Column::SenseId.is_in(chunk.iter().copied()))
            .group_by(sentence::Column::SenseId)
            .into_tuple()
            .all(db)
            .await?;
        counts.extend(
            rows.into_iter()
                .map(|(id, count)| (id, u32::try_from(count).unwrap_or(u32::MAX))),
        );
    }
    Ok(counts)
}

/// Replaces all sentences of a sense, keeping the given order.
pub async fn replace_sentences(
    db: &impl ConnectionTrait,
    sense_id: i64,
    sentences: &[SentenceContent],
) -> Result<(), DbErr> {
    sentence::Entity::delete_many()
        .filter(sentence::Column::SenseId.eq(sense_id))
        .exec(db)
        .await?;
    for sentence in sentences {
        insert_sentence(
            db,
            sense_id,
            &sentence.marked,
            sentence.translation.as_deref(),
        )
        .await?;
    }
    Ok(())
}

/// All sentences, grouped by sense in creation order.
#[cfg(test)]
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
