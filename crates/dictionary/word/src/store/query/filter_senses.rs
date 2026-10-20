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
