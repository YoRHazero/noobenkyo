use sea_orm::{ConnectionTrait, DbErr};

use crate::SenseKey;
use crate::store::by_key;
use crate::store::tables::sense;

/// Deletes a sense with its labels and sentences; its word and reading stay,
/// even when nothing is left below them. `false` if the sense is already gone.
pub async fn delete_sense(db: &impl ConnectionTrait, key: &SenseKey) -> Result<bool, DbErr> {
    let Some(found) = by_key::find_sense(db, key).await? else {
        return Ok(false);
    };
    sense::delete_sense(db, found.id).await?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::tables::reading::all_readings;
    use crate::store::tables::sense::all_senses;
    use crate::store::test_db::{memory_db, sense_of};

    #[tokio::test]
    async fn deletes_the_sense_and_keeps_its_reading() {
        let db = memory_db().await;
        sense_of(&db, "明日", "あした", "明天").await;
        let key = SenseKey {
            writing: "明日".into(),
            origin: String::new(),
            kana: "あした".into(),
            primary_gloss: "明天".into(),
        };
        assert!(delete_sense(&db, &key).await.unwrap());
        assert!(all_senses(&db).await.unwrap().is_empty());
        assert_eq!(all_readings(&db).await.unwrap().len(), 1);
        assert!(!delete_sense(&db, &key).await.unwrap());
    }
}
