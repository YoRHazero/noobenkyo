//! One sense in full, for the dictionary page and for entering a zone.

use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter};
use serde::Serialize;

use crate::entity::sense as sense_entity;
use crate::store::by_key;
use crate::store::tables::{label, reading, sense, sentence};
use crate::{Progress, SenseKey, SentenceContent};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct SenseDetail {
    pub key: SenseKey,
    /// All glosses, the primary gloss first.
    pub glosses: Vec<String>,
    pub labels: Vec<String>,
    pub progress: Progress,
    /// Marked text and translation, in creation order.
    pub sentences: Vec<SentenceContent>,
    /// Senses under the same reading, this one included.
    pub reading_senses: u32,
    /// Readings under the same word.
    pub word_readings: u32,
    /// Senses under the same word.
    pub word_senses: u32,
}

/// Everything about one sense; `None` if it is gone.
pub async fn sense_detail(
    db: &impl ConnectionTrait,
    key: &SenseKey,
) -> Result<Option<SenseDetail>, DbErr> {
    let Some(model) = by_key::find_sense(db, key).await? else {
        return Ok(None);
    };
    let sentences = sentence::sentences_of_sense(db, model.id)
        .await?
        .into_iter()
        .map(|row| SentenceContent {
            marked: row.marked,
            translation: row.translation,
        })
        .collect();
    let reading_senses = sense::senses_of_reading(db, model.reading_id).await?.len();
    let Some(reading) = reading::readings_by_ids(db, &[model.reading_id])
        .await?
        .into_iter()
        .next()
    else {
        return Ok(None);
    };
    let word_readings = reading::readings_of_word(db, reading.word_id).await?;
    let reading_ids: Vec<i64> = word_readings.iter().map(|r| r.id).collect();
    let word_senses = sense_entity::Entity::find()
        .filter(sense_entity::Column::ReadingId.is_in(reading_ids))
        .count(db)
        .await?;
    Ok(Some(SenseDetail {
        key: key.clone(),
        glosses: sense::glosses(&model),
        labels: label::labels_of_sense(db, model.id).await?,
        progress: sense::progress(&model),
        sentences,
        reading_senses: to_u32(reading_senses),
        word_readings: to_u32(word_readings.len()),
        word_senses: u32::try_from(word_senses).unwrap_or(u32::MAX),
    }))
}

fn to_u32(count: usize) -> u32 {
    u32::try_from(count).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::tables::label::add_labels;
    use crate::store::tables::sentence::insert_sentence;
    use crate::store::test_db::{memory_db, progress_at, sense_of};

    fn key(kana: &str, gloss: &str) -> SenseKey {
        SenseKey {
            writing: "明日".into(),
            origin: String::new(),
            kana: kana.into(),
            primary_gloss: gloss.into(),
        }
    }

    #[tokio::test]
    async fn loads_everything_about_a_sense() {
        let db = memory_db().await;
        let id = sense_of(&db, "明日", "あした", "明天").await;
        sense_of(&db, "明日", "あした", "将来").await;
        sense_of(&db, "明日", "あす", "明天").await;
        add_labels(&db, id, &["N5".to_owned()]).await.unwrap();
        insert_sentence(&db, id, "[[{明日|あした}]]", Some("明天"))
            .await
            .unwrap();

        let detail = sense_detail(&db, &key("あした", "明天"))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(detail.glosses, ["明天"]);
        assert_eq!(detail.labels, ["N5"]);
        assert_eq!(detail.progress, progress_at(0, 0, 0));
        assert_eq!(
            detail.sentences,
            [SentenceContent {
                marked: "[[{明日|あした}]]".into(),
                translation: Some("明天".into()),
            }]
        );
        assert_eq!(
            (
                detail.reading_senses,
                detail.word_readings,
                detail.word_senses
            ),
            (2, 2, 3)
        );
    }

    #[tokio::test]
    async fn returns_none_for_missing_sense() {
        let db = memory_db().await;
        assert!(
            sense_detail(&db, &key("あした", "明天"))
                .await
                .unwrap()
                .is_none()
        );
    }
}
