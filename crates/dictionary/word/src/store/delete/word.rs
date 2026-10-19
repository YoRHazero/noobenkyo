use sea_orm::{ConnectionTrait, DbErr};

use crate::WordKey;
use crate::store::by_key;
use crate::store::tables::word;

/// Deletes a word with its readings, senses, labels and sentences.
/// `false` if the word is already gone.
pub async fn delete_word(db: &impl ConnectionTrait, key: &WordKey) -> Result<bool, DbErr> {
    let Some(found) = by_key::find_word(db, key).await? else {
        return Ok(false);
    };
    word::delete_word(db, found.id).await?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::tables::sense::all_senses;
    use crate::store::tables::sentence::{all_sentences, insert_sentence};
    use crate::store::tables::word::all_words;
    use crate::store::test_db::{memory_db, sense_of};

    #[tokio::test]
    async fn deletes_everything_below() {
        let db = memory_db().await;
        let ashita = sense_of(&db, "明日", "あした", "明天").await;
        sense_of(&db, "明日", "あす", "明天").await;
        insert_sentence(&db, ashita, "[[{明日|あした}]]", None)
            .await
            .unwrap();
        sense_of(&db, "上", "うえ", "上面").await;
        let key = WordKey {
            writing: "明日".into(),
            origin: String::new(),
        };
        assert!(delete_word(&db, &key).await.unwrap());
        assert_eq!(all_words(&db).await.unwrap().len(), 1);
        assert_eq!(all_senses(&db).await.unwrap().len(), 1);
        assert!(all_sentences(&db).await.unwrap().is_empty());
        assert!(!delete_word(&db, &key).await.unwrap());
    }
}
