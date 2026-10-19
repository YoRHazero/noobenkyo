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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::tables::reading::insert_reading;
    use crate::store::tables::sense::set_progress;
    use crate::store::tables::sentence::insert_sentence;
    use crate::store::tables::word::insert_word;
    use crate::store::test_db::{memory_db, progress_at, sense_of};

    /// 明日: あした (sense with a sentence, learned), あす (sense without sentences);
    /// 上: うえ (reading only); パソコン (word only, origin "personal computer").
    async fn fixture(db: &impl ConnectionTrait) {
        let ashita = sense_of(db, "明日", "あした", "明天").await;
        insert_sentence(db, ashita, "[[{明日|あした}]]", None)
            .await
            .unwrap();
        set_progress(db, ashita, &progress_at(1, 1, 0))
            .await
            .unwrap();
        sense_of(db, "明日", "あす", "明天").await;
        let ue = insert_word(db, "上", "", "").await.unwrap();
        insert_reading(db, ue.id, "うえ").await.unwrap();
        insert_word(db, "パソコン", "personal computer", "")
            .await
            .unwrap();
    }

    async fn levels(db: &impl ConnectionTrait, filter: ContentFilter) -> Vec<(i64, bool, bool)> {
        matching_rows(db, &filter)
            .await
            .unwrap()
            .into_iter()
            .map(|row| {
                (
                    row.word_id,
                    row.reading_id.is_some(),
                    row.sense_id.is_some(),
                )
            })
            .collect()
    }

    #[tokio::test]
    async fn lists_every_row_word_by_word() {
        let db = memory_db().await;
        fixture(&db).await;
        assert_eq!(
            levels(&db, ContentFilter::default()).await,
            [
                (1, true, true),
                (1, true, true),
                (2, true, false),
                (3, false, false)
            ]
        );
    }

    #[tokio::test]
    async fn filters_by_completeness() {
        let db = memory_db().await;
        fixture(&db).await;
        let only = |levels: Vec<Completeness>| ContentFilter {
            completeness: levels,
            ..ContentFilter::default()
        };
        assert_eq!(
            levels(&db, only(vec![Completeness::WithSentence]))
                .await
                .len(),
            1
        );
        assert_eq!(
            levels(&db, only(vec![Completeness::NoSentence])).await,
            [(1, true, true)]
        );
        assert_eq!(
            levels(
                &db,
                only(vec![Completeness::WordOnly, Completeness::ReadingOnly])
            )
            .await,
            [(2, true, false), (3, false, false)]
        );
    }

    #[tokio::test]
    async fn progress_keeps_senses_only() {
        let db = memory_db().await;
        fixture(&db).await;
        let progress = |state| ContentFilter {
            progress: state,
            ..ContentFilter::default()
        };
        assert_eq!(
            levels(&db, progress(ProgressState::Untouched)).await.len(),
            1
        );
        assert_eq!(levels(&db, progress(ProgressState::Touched)).await.len(), 1);
    }

    #[tokio::test]
    async fn text_matches_each_layer() {
        let db = memory_db().await;
        fixture(&db).await;
        let text = |text: &str| ContentFilter {
            text: Some(text.into()),
            ..ContentFilter::default()
        };
        assert_eq!(levels(&db, text("明天")).await.len(), 2);
        assert_eq!(levels(&db, text("アス")).await, [(1, true, true)]);
        assert_eq!(levels(&db, text("うえ")).await, [(2, true, false)]);
        assert_eq!(levels(&db, text("computer")).await, [(3, false, false)]);
        assert_eq!(levels(&db, text(" ")).await.len(), 4);
    }
}
