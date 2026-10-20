//! The rows of the dictionary that match a filter, without their content.

use sea_orm::sea_query::Query;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, JoinType, QueryFilter, QuerySelect,
    RelationTrait,
};

use super::sense_condition::sense_condition;
use super::text_condition::{reading_matches, word_matches};
use super::{Completeness, ContentFilter, ProgressState};
use crate::entity::{reading, sense, word};

/// One row: a sense, or a reading or word with nothing below it. Ordering
/// by these ids puts a word's rows together, each layer in creation order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Row {
    pub word_id: i64,
    pub reading_id: Option<i64>,
    pub sense_id: Option<i64>,
}

/// Rows matching the filter, word by word in creation order.
pub async fn matching_rows(
    db: &impl ConnectionTrait,
    filter: &ContentFilter,
) -> Result<Vec<Row>, DbErr> {
    let mut rows = sense_rows(db, filter).await?;
    if filter.progress == ProgressState::Any {
        if filter.allows(Completeness::ReadingOnly) {
            rows.extend(reading_rows(db, filter).await?);
        }
        if filter.allows(Completeness::WordOnly) {
            rows.extend(word_rows(db, filter).await?);
        }
    }
    rows.sort();
    Ok(rows)
}

async fn sense_rows(db: &impl ConnectionTrait, filter: &ContentFilter) -> Result<Vec<Row>, DbErr> {
    let Some(condition) = sense_condition(filter) else {
        return Ok(Vec::new());
    };
    let ids: Vec<(i64, i64, i64)> = sense::Entity::find()
        .select_only()
        .column_as(word::Column::Id, "word_id")
        .column_as(reading::Column::Id, "reading_id")
        .column_as(sense::Column::Id, "sense_id")
        .join(JoinType::InnerJoin, sense::Relation::Reading.def())
        .join(JoinType::InnerJoin, reading::Relation::Word.def())
        .filter(condition)
        .into_tuple()
        .all(db)
        .await?;
    Ok(ids
        .into_iter()
        .map(|(word_id, reading_id, sense_id)| Row {
            word_id,
            reading_id: Some(reading_id),
            sense_id: Some(sense_id),
        })
        .collect())
}

/// Readings without senses.
async fn reading_rows(
    db: &impl ConnectionTrait,
    filter: &ContentFilter,
) -> Result<Vec<Row>, DbErr> {
    let mut query = reading::Entity::find()
        .select_only()
        .column_as(word::Column::Id, "word_id")
        .column_as(reading::Column::Id, "reading_id")
        .join(JoinType::InnerJoin, reading::Relation::Word.def())
        .filter(
            reading::Column::Id.not_in_subquery(
                Query::select()
                    .column(sense::Column::ReadingId)
                    .from(sense::Entity)
                    .to_owned(),
            ),
        );
    if let Some(text) = filter.text() {
        query = query.filter(reading_matches(text));
    }
    let ids: Vec<(i64, i64)> = query.into_tuple().all(db).await?;
    Ok(ids
        .into_iter()
        .map(|(word_id, reading_id)| Row {
            word_id,
            reading_id: Some(reading_id),
            sense_id: None,
        })
        .collect())
}

/// Words without readings.
async fn word_rows(db: &impl ConnectionTrait, filter: &ContentFilter) -> Result<Vec<Row>, DbErr> {
    let mut query = word::Entity::find()
        .select_only()
        .column(word::Column::Id)
        .filter(
            word::Column::Id.not_in_subquery(
                Query::select()
                    .column(reading::Column::WordId)
                    .from(reading::Entity)
                    .to_owned(),
            ),
        );
    if let Some(text) = filter.text() {
        query = query.filter(word_matches(text));
    }
    let ids: Vec<i64> = query.into_tuple().all(db).await?;
    Ok(ids
        .into_iter()
        .map(|word_id| Row {
            word_id,
            reading_id: None,
            sense_id: None,
        })
        .collect())
}
