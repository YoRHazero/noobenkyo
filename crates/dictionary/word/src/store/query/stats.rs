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
