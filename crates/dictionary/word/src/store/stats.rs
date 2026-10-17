use sea_orm::sea_query::Query;
use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter};
use serde::Serialize;

use crate::entity::{sense, sentence, word};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct DictionaryStats {
    pub words: u32,
    pub senses: u32,
    pub senses_with_sentences: u32,
    /// Senses with zero passes and zero lapses.
    pub unlearned_senses: u32,
}

pub async fn count_stats(db: &impl ConnectionTrait) -> Result<DictionaryStats, DbErr> {
    let words = word::Entity::find().count(db).await?;
    let senses = sense::Entity::find().count(db).await?;
    let senses_with_sentences = sense::Entity::find()
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
        senses: to_u32(senses),
        senses_with_sentences: to_u32(senses_with_sentences),
        unlearned_senses: to_u32(unlearned_senses),
    })
}

fn to_u32(count: u64) -> u32 {
    u32::try_from(count).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::sense::set_progress;
    use crate::store::sentence::insert_sentence;
    use crate::store::test_db::{memory_db, progress_at, sense_of};

    #[tokio::test]
    async fn counts_words_and_senses() {
        let db = memory_db().await;
        let ashita = sense_of(&db, "明日", "あした", "明天").await;
        let asu = sense_of(&db, "明日", "あす", "明天").await;
        insert_sentence(&db, ashita, "[[{明日|あした}]]", None)
            .await
            .unwrap();
        set_progress(&db, asu, &progress_at(1, 1, 0)).await.unwrap();
        assert_eq!(
            count_stats(&db).await.unwrap(),
            DictionaryStats {
                words: 1,
                senses: 2,
                senses_with_sentences: 1,
                unlearned_senses: 1,
            }
        );
    }
}
