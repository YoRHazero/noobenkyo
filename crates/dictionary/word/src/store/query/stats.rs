use sea_orm::sea_query::{Expr, ExprTrait, Query};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, JoinType, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, RelationTrait,
};
use serde::Serialize;

use super::sense_condition::sense_condition;
use super::{Completeness, ContentFilter, ProgressState};
use crate::entity::{reading, sense, sentence, word};

/// Counts of the dictionary: rows by how far they are written, and senses
/// with sentences by their progress.
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
    /// Senses with sentences that are untouched, as the progress filter means it.
    pub untouched: u32,
    /// Senses with sentences that are touched, by stage in ascending order;
    /// stages without senses are absent.
    pub stages: Vec<StageCount>,
}

/// How many senses are at a stage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct StageCount {
    pub stage: u32,
    pub senses: u32,
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
    let untouched = sense::Entity::find()
        .join(JoinType::InnerJoin, sense::Relation::Reading.def())
        .join(JoinType::InnerJoin, reading::Relation::Word.def())
        .filter(with_sentences(ProgressState::Untouched))
        .count(db)
        .await?;
    Ok(DictionaryStats {
        words: to_u32(words),
        word_only: to_u32(word_only),
        reading_only: to_u32(reading_only),
        no_sentence: to_u32(senses - with_sentence),
        with_sentence: to_u32(with_sentence),
        untouched: to_u32(untouched),
        stages: touched_by_stage(db).await?,
    })
}

async fn touched_by_stage(db: &impl ConnectionTrait) -> Result<Vec<StageCount>, DbErr> {
    let rows: Vec<(i64, i64)> = sense::Entity::find()
        .select_only()
        .column(sense::Column::Stage)
        .column_as(
            Expr::col((sense::Entity, sense::Column::Id)).count(),
            "senses",
        )
        .join(JoinType::InnerJoin, sense::Relation::Reading.def())
        .join(JoinType::InnerJoin, reading::Relation::Word.def())
        .filter(with_sentences(ProgressState::Touched))
        .group_by(sense::Column::Stage)
        .order_by_asc(sense::Column::Stage)
        .into_tuple()
        .all(db)
        .await?;
    Ok(rows
        .into_iter()
        .map(|(stage, senses)| StageCount {
            stage: u32::try_from(stage).unwrap_or(0),
            senses: u32::try_from(senses).unwrap_or(u32::MAX),
        })
        .collect())
}

/// Senses with sentences in a progress state, by the same condition the
/// filters use.
fn with_sentences(progress: ProgressState) -> Condition {
    let filter = ContentFilter {
        text: None,
        completeness: vec![Completeness::WithSentence],
        progress,
    };
    sense_condition(&filter).expect("a filter allowing senses always has a condition")
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
                untouched: 1,
                stages: Vec::new(),
            }
        );
    }

    #[tokio::test]
    async fn counts_senses_with_sentences_by_progress() {
        let db = memory_db().await;
        let mut with_sentence = Vec::new();
        for kana in ["あ", "い", "う", "え"] {
            let id = sense_of(&db, "語", kana, "义").await;
            insert_sentence(&db, id, "[[{語|ご}]]", None).await.unwrap();
            with_sentence.push(id);
        }
        // Stage 0 after a lapse still counts as touched.
        set_progress(&db, with_sentence[1], &progress_at(0, 1, 1))
            .await
            .unwrap();
        set_progress(&db, with_sentence[2], &progress_at(2, 1, 0))
            .await
            .unwrap();
        set_progress(&db, with_sentence[3], &progress_at(2, 3, 0))
            .await
            .unwrap();
        // Without sentences: counted in neither.
        let bare = sense_of(&db, "語", "お", "义").await;
        set_progress(&db, bare, &progress_at(5, 1, 0))
            .await
            .unwrap();
        sense_of(&db, "語", "か", "义").await;

        let stats = count_stats(&db).await.unwrap();
        assert_eq!(stats.untouched, 1);
        assert_eq!(
            stats.stages,
            [
                StageCount {
                    stage: 0,
                    senses: 1
                },
                StageCount {
                    stage: 2,
                    senses: 2
                }
            ]
        );
    }
}
