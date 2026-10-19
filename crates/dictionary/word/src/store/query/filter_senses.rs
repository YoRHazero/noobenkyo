//! Senses matching a filter, with their progress.

use std::collections::HashSet;

use chrono::{DateTime, FixedOffset};
use sea_orm::sea_query::Expr;
use sea_orm::{
    ConnectionTrait, DbErr, EntityTrait, FromQueryResult, JoinType, Order, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait,
};

use super::sense_condition::sense_condition;
use super::{ContentFilter, SenseOrder};
use crate::entity::{reading, sense, word};
use crate::{Progress, SenseKey};

/// A sense and its progress.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SenseProgress {
    pub key: SenseKey,
    pub progress: Progress,
}

#[derive(FromQueryResult)]
struct ProgressRow {
    writing: String,
    origin: String,
    kana: String,
    primary_gloss: String,
    stage: i64,
    passes: i64,
    lapses: i64,
    modified_at: DateTime<FixedOffset>,
}

/// Senses matching the filter, except the excluded ones, in the given
/// order, at most `limit` of them.
pub async fn filter_senses(
    db: &impl ConnectionTrait,
    filter: &ContentFilter,
    exclude: &[SenseKey],
    order: SenseOrder,
    limit: Option<u32>,
) -> Result<Vec<SenseProgress>, DbErr> {
    let Some(condition) = sense_condition(filter) else {
        return Ok(Vec::new());
    };
    let mut query = sense::Entity::find()
        .select_only()
        .column(word::Column::Writing)
        .column(word::Column::Origin)
        .column(reading::Column::Kana)
        .column(sense::Column::PrimaryGloss)
        .column(sense::Column::Stage)
        .column(sense::Column::Passes)
        .column(sense::Column::Lapses)
        .column(sense::Column::ModifiedAt)
        .join(JoinType::InnerJoin, sense::Relation::Reading.def())
        .join(JoinType::InnerJoin, reading::Relation::Word.def())
        .filter(condition);
    query = match order {
        SenseOrder::Creation => query.order_by_asc(sense::Column::Id),
        SenseOrder::Random => query.order_by(Expr::cust("RANDOM()"), Order::Asc),
    };
    if let Some(limit) = limit {
        // Fetch extra rows for the excluded ones, so dropping them still leaves `limit`.
        let extra = u64::try_from(exclude.len()).unwrap_or(u64::MAX);
        query = query.limit(u64::from(limit).saturating_add(extra));
    }
    let excluded: HashSet<&SenseKey> = exclude.iter().collect();
    Ok(query
        .into_model::<ProgressRow>()
        .all(db)
        .await?
        .into_iter()
        .map(to_sense_progress)
        .filter(|found| !excluded.contains(&found.key))
        .take(limit.map_or(usize::MAX, |l| usize::try_from(l).unwrap_or(usize::MAX)))
        .collect())
}

fn to_sense_progress(row: ProgressRow) -> SenseProgress {
    SenseProgress {
        key: SenseKey {
            writing: row.writing,
            origin: row.origin,
            kana: row.kana,
            primary_gloss: row.primary_gloss,
        },
        progress: Progress {
            stage: u32::try_from(row.stage).unwrap_or(0),
            passes: u32::try_from(row.passes).unwrap_or(0),
            lapses: u32::try_from(row.lapses).unwrap_or(0),
            modified_at: row.modified_at,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::query::{Completeness, ProgressState};
    use crate::store::tables::sense::set_progress;
    use crate::store::tables::sentence::insert_sentence;
    use crate::store::test_db::{memory_db, progress_at, sense_of};

    fn with_sentences() -> ContentFilter {
        ContentFilter {
            completeness: vec![Completeness::WithSentence],
            ..ContentFilter::default()
        }
    }

    fn kana(found: &[SenseProgress]) -> Vec<&str> {
        found.iter().map(|s| s.key.kana.as_str()).collect()
    }

    async fn fixture(db: &impl ConnectionTrait) {
        for kana in ["あ", "い", "う", "え"] {
            let id = sense_of(db, "語", kana, "义").await;
            insert_sentence(db, id, "[[{語|ご}]]", None).await.unwrap();
        }
        sense_of(db, "語", "お", "义").await;
    }

    #[tokio::test]
    async fn takes_matching_senses_in_creation_order() {
        let db = memory_db().await;
        fixture(&db).await;
        let found = filter_senses(&db, &with_sentences(), &[], SenseOrder::Creation, Some(2))
            .await
            .unwrap();
        assert_eq!(kana(&found), ["あ", "い"]);
        assert_eq!(found[0].progress, progress_at(0, 0, 0));
        let all = filter_senses(&db, &with_sentences(), &[], SenseOrder::Creation, None)
            .await
            .unwrap();
        assert_eq!(all.len(), 4);
    }

    #[tokio::test]
    async fn excluded_senses_do_not_shorten_the_result() {
        let db = memory_db().await;
        fixture(&db).await;
        let exclude: Vec<SenseKey> =
            filter_senses(&db, &with_sentences(), &[], SenseOrder::Creation, Some(2))
                .await
                .unwrap()
                .into_iter()
                .map(|s| s.key)
                .collect();
        let found = filter_senses(
            &db,
            &with_sentences(),
            &exclude,
            SenseOrder::Random,
            Some(2),
        )
        .await
        .unwrap();
        let mut got = kana(&found);
        got.sort_unstable();
        assert_eq!(got, ["う", "え"]);
    }

    #[tokio::test]
    async fn filters_by_progress() {
        let db = memory_db().await;
        let learned = sense_of(&db, "語", "あ", "义").await;
        set_progress(&db, learned, &progress_at(2, 1, 0))
            .await
            .unwrap();
        sense_of(&db, "語", "い", "义").await;
        let filter = ContentFilter {
            progress: ProgressState::Untouched,
            ..ContentFilter::default()
        };
        let found = filter_senses(&db, &filter, &[], SenseOrder::Creation, None)
            .await
            .unwrap();
        assert_eq!(kana(&found), ["い"]);
    }
}
