use sea_orm::{ConnectionTrait, DbErr};

use crate::rules::marked;
use crate::store::{sense, sense_by_key, sentence};
use crate::{SenseKey, SenseSnapshot, SentenceSnapshot};

/// Loads everything study needs about a sense. `None` if the sense is gone.
pub async fn load_snapshot(
    db: &impl ConnectionTrait,
    key: &SenseKey,
) -> Result<Option<SenseSnapshot>, DbErr> {
    let Some(model) = sense_by_key::find_sense(db, key).await? else {
        return Ok(None);
    };
    let sentences = sentence::sentences_of_sense(db, model.id)
        .await?
        .into_iter()
        .filter_map(|row| {
            let segments = marked::parse_checked(&row.marked).ok()?;
            Some(SentenceSnapshot {
                target_kana: marked::target_kana(&segments)?,
                marked: row.marked,
                translation: row.translation,
            })
        })
        .collect();
    let progress = sense::progress(&model);
    Ok(Some(SenseSnapshot {
        key: key.clone(),
        glosses: sense::glosses(&model),
        sentences,
        stage: progress.stage,
        passes: progress.passes,
        lapses: progress.lapses,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::sentence::insert_sentence;
    use crate::store::test_db::{memory_db, sense_of};

    fn key() -> SenseKey {
        SenseKey {
            writing: "諦める".into(),
            origin: String::new(),
            kana: "あきらめる".into(),
            primary_gloss: "放弃".into(),
        }
    }

    #[tokio::test]
    async fn loads_glosses_sentences_and_target_kana() {
        let db = memory_db().await;
        let id = sense_of(&db, "諦める", "あきらめる", "放弃").await;
        insert_sentence(&db, id, "[[{諦|あきら}めない]]で", Some("别放弃"))
            .await
            .unwrap();

        let snapshot = load_snapshot(&db, &key()).await.unwrap().unwrap();
        assert_eq!(snapshot.glosses, ["放弃"]);
        assert_eq!(
            snapshot.sentences,
            [SentenceSnapshot {
                marked: "[[{諦|あきら}めない]]で".into(),
                translation: Some("别放弃".into()),
                target_kana: "あきらめない".into(),
            }]
        );
    }

    #[tokio::test]
    async fn returns_none_for_missing_sense() {
        let db = memory_db().await;
        assert!(load_snapshot(&db, &key()).await.unwrap().is_none());
    }
}
