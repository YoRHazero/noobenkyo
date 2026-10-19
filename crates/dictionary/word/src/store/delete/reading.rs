use sea_orm::{ConnectionTrait, DbErr};

use crate::ReadingKey;
use crate::store::by_key;
use crate::store::tables::reading;

/// Deletes a reading with its senses, labels and sentences; the word stays.
/// `false` if the reading is already gone.
pub async fn delete_reading(db: &impl ConnectionTrait, key: &ReadingKey) -> Result<bool, DbErr> {
    let Some(found) = by_key::find_reading(db, key).await? else {
        return Ok(false);
    };
    reading::delete_reading(db, found.id).await?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::tables::reading::all_readings;
    use crate::store::tables::sense::all_senses;
    use crate::store::tables::word::all_words;
    use crate::store::test_db::{memory_db, sense_of};

    #[tokio::test]
    async fn deletes_senses_and_keeps_the_word() {
        let db = memory_db().await;
        sense_of(&db, "諦める", "あきらねる", "放弃").await;
        sense_of(&db, "諦める", "あきらめる", "死心").await;
        let key = ReadingKey {
            writing: "諦める".into(),
            origin: String::new(),
            kana: "あきらねる".into(),
        };
        assert!(delete_reading(&db, &key).await.unwrap());
        assert_eq!(all_words(&db).await.unwrap().len(), 1);
        assert_eq!(all_readings(&db).await.unwrap().len(), 1);
        assert_eq!(all_senses(&db).await.unwrap()[0].primary_gloss, "死心");
        assert!(!delete_reading(&db, &key).await.unwrap());
    }
}
