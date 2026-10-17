use sea_orm::{ConnectionTrait, DbErr};

use crate::store::{sense, sense_by_key};
use crate::{Progress, SenseKey};

/// Overwrites the progress of the sense with the given key.
/// Returns `false` if the sense no longer exists.
pub async fn write_progress(
    db: &impl ConnectionTrait,
    key: &SenseKey,
    progress: &Progress,
) -> Result<bool, DbErr> {
    let Some(model) = sense_by_key::find_sense(db, key).await? else {
        return Ok(false);
    };
    sense::set_progress(db, model.id, progress).await?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::sense::{all_senses, progress};
    use crate::store::test_db::{memory_db, progress_at, sense_of};

    fn key(gloss: &str) -> SenseKey {
        SenseKey {
            writing: "明日".into(),
            origin: String::new(),
            kana: "あした".into(),
            primary_gloss: gloss.into(),
        }
    }

    #[tokio::test]
    async fn overwrites_existing_progress() {
        let db = memory_db().await;
        sense_of(&db, "明日", "あした", "明天").await;
        assert!(
            write_progress(&db, &key("明天"), &progress_at(2, 3, 1))
                .await
                .unwrap()
        );
        assert_eq!(
            progress(&all_senses(&db).await.unwrap()[0]),
            progress_at(2, 3, 1)
        );
    }

    #[tokio::test]
    async fn skips_missing_sense() {
        let db = memory_db().await;
        assert!(
            !write_progress(&db, &key("明天"), &progress_at(2, 3, 1))
                .await
                .unwrap()
        );
    }
}
