use sea_orm::{ConnectionTrait, DbErr};

use crate::SenseKey;
use crate::rules::sentence;
use crate::store::by_key;
use crate::store::tables::sentence as sentence_table;

/// Deletes the sentence of a sense whose plain text equals that of
/// `marked`. `false` if there is no such sentence.
pub async fn delete_sentence(
    db: &impl ConnectionTrait,
    key: &SenseKey,
    marked: &str,
) -> Result<bool, DbErr> {
    let Some(sense) = by_key::find_sense(db, key).await? else {
        return Ok(false);
    };
    let Some(plain) = plain_of(marked) else {
        return Ok(false);
    };
    for row in sentence_table::sentences_of_sense(db, sense.id).await? {
        if plain_of(&row.marked).as_deref() == Some(plain.as_str()) {
            sentence_table::delete_sentence(db, row.id).await?;
            return Ok(true);
        }
    }
    Ok(false)
}

fn plain_of(marked: &str) -> Option<String> {
    sentence::parse(marked)
        .ok()
        .map(|segments| sentence::plain_text(&segments))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::tables::sentence::{all_sentences, insert_sentence};
    use crate::store::test_db::{memory_db, sense_of};

    fn key() -> SenseKey {
        SenseKey {
            writing: "明日".into(),
            origin: String::new(),
            kana: "あした".into(),
            primary_gloss: "明天".into(),
        }
    }

    #[tokio::test]
    async fn deletes_the_sentence_with_the_same_plain_text() {
        let db = memory_db().await;
        let id = sense_of(&db, "明日", "あした", "明天").await;
        insert_sentence(&db, id, "[[{明日|あした}]]だ", None)
            .await
            .unwrap();
        insert_sentence(&db, id, "[[{明日|あした}]]です", None)
            .await
            .unwrap();
        assert!(
            delete_sentence(&db, &key(), "[[{明日|あした}]]だ")
                .await
                .unwrap()
        );
        let rest = all_sentences(&db).await.unwrap();
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0].marked, "[[{明日|あした}]]です");
        assert!(
            !delete_sentence(&db, &key(), "[[{明日|あした}]]だ")
                .await
                .unwrap()
        );
    }
}
