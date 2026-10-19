use sea_orm::sea_query::Query;
use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter};
use serde::Serialize;

use crate::entity::{reading, sense, sentence, word};

/// Counts for the dictionary page: rows by how far they are written.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct DictionaryStats {
    pub words: u32,
    /// Words without readings.
    pub word_only: u32,
    /// Readings without senses.
    pub reading_only: u32,
    /// Senses without sentences.
    pub no_sentence: u32,
    /// Senses with sentences.
    pub with_sentence: u32,
    /// Senses with zero passes and zero lapses.
    pub unlearned_senses: u32,
}

pub async fn count_stats(db: &impl ConnectionTrait) -> Result<DictionaryStats, DbErr> {
    let words = word::Entity::find().count(db).await?;
    let word_only = word::Entity::find()
        .filter(
            word::Column::Id.not_in_subquery(
                Query::select()
                    .column(reading::Column::WordId)
                    .from(reading::Entity)
                    .to_owned(),
            ),
        )
        .count(db)
        .await?;
    let reading_only = reading::Entity::find()
        .filter(
            reading::Column::Id.not_in_subquery(
                Query::select()
                    .column(sense::Column::ReadingId)
                    .from(sense::Entity)
                    .to_owned(),
            ),
        )
        .count(db)
        .await?;
    let senses = sense::Entity::find().count(db).await?;
    let with_sentence = sense::Entity::find()
        .filter(
            sense::Column::Id.in_subquery(
                Query::select()
                    .column(sentence::Column::SenseId)
                    .from(sentence::Entity)
                    .to_owned(),
            ),
        )
        .count(db)
        .await?;
    let unlearned_senses = sense::Entity::find()
        .filter(sense::Column::Passes.eq(0))
        .filter(sense::Column::Lapses.eq(0))
        .count(db)
        .await?;
    Ok(DictionaryStats {
        words: to_u32(words),
        word_only: to_u32(word_only),
        reading_only: to_u32(reading_only),
        no_sentence: to_u32(senses - with_sentence),
        with_sentence: to_u32(with_sentence),
        unlearned_senses: to_u32(unlearned_senses),
    })
}

fn to_u32(count: u64) -> u32 {
    u32::try_from(count).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::tables::reading::insert_reading;
    use crate::store::tables::sense::set_progress;
    use crate::store::tables::sentence::insert_sentence;
    use crate::store::tables::word::insert_word;
    use crate::store::test_db::{memory_db, progress_at, sense_of};

    #[tokio::test]
    async fn counts_rows_by_level() {
        let db = memory_db().await;
        let ashita = sense_of(&db, "明日", "あした", "明天").await;
        let asu = sense_of(&db, "明日", "あす", "明天").await;
        insert_sentence(&db, ashita, "[[{明日|あした}]]", None)
            .await
            .unwrap();
        set_progress(&db, asu, &progress_at(1, 1, 0)).await.unwrap();
        let ue = insert_word(&db, "上", "", "").await.unwrap();
        insert_reading(&db, ue.id, "うえ").await.unwrap();
        insert_word(&db, "パソコン", "", "").await.unwrap();
        assert_eq!(
            count_stats(&db).await.unwrap(),
            DictionaryStats {
                words: 3,
                word_only: 1,
                reading_only: 1,
                no_sentence: 1,
                with_sentence: 1,
                unlearned_senses: 1,
            }
        );
    }
}
