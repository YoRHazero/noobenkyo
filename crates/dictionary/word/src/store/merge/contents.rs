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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::merge::SenseOutcome;
    use crate::store::tables::sense::all_senses;
    use crate::store::test_db::{memory_db, time};
    use crate::{ReadingContent, SenseContent};

    fn content(kana: &str) -> Content {
        Content {
            writing: "明日".into(),
            origin: String::new(),
            stratum: String::new(),
            reading: Some(ReadingContent {
                kana: kana.into(),
                sense: Some(SenseContent {
                    glosses: vec!["明天".into()],
                    labels: vec![],
                    sentences: vec![],
                    progress: None,
                }),
            }),
        }
    }

    #[tokio::test]
    async fn later_contents_see_earlier_ones() {
        let db = memory_db().await;
        let ashita = content("あした");
        let asu = content("あす");
        let outcomes = merge_contents(
            &db,
            &[&ashita, &asu, &ashita],
            time("2026-10-18T09:00:00+08:00"),
        )
        .await
        .unwrap();
        assert!(outcomes[0].word_added);
        assert!(!outcomes[1].word_added && outcomes[1].reading_added);
        assert_eq!(outcomes[2].sense, SenseOutcome::Merged);
        assert_eq!(all_senses(&db).await.unwrap().len(), 2);
    }
}
