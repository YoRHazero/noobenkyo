use chrono::{DateTime, FixedOffset};
use sea_orm::{DatabaseConnection, DbErr, TransactionTrait};

use super::MergeOutcome;
use super::layers::merge_content;
use crate::Content;

/// Merges contents in order, in one transaction, so later contents see what
/// earlier ones created. Returns what each one changed, in the same order.
pub async fn merge_contents(
    db: &DatabaseConnection,
    contents: &[&Content],
    now: DateTime<FixedOffset>,
) -> Result<Vec<MergeOutcome>, DbErr> {
    let txn = db.begin().await?;
    let mut outcomes = Vec::with_capacity(contents.len());
    for content in contents {
        outcomes.push(merge_content(&txn, content, now).await?);
    }
    txn.commit().await?;
    Ok(outcomes)
}
