//! Deleting a word, reading, sense or sentence, with everything below it.

use dictionary_word::store::delete::reading::delete_reading;
use dictionary_word::store::delete::sense::delete_sense;
use dictionary_word::store::delete::sentence::delete_sentence;
use dictionary_word::store::delete::word::delete_word;
use dictionary_word::{ReadingKey, WordKey};

use crate::support::{
    all_readings, all_senses, all_sentences, all_words, memory_db, merge, sense, sense_key,
    sentence, with_sentences,
};

#[tokio::test]
async fn a_word_goes_with_everything_below() {
    let db = memory_db().await;
    merge(
        &db,
        &[
            with_sentences(
                sense("明日", "あした", &["明天"]),
                &[sentence("[[{明日|あした}]]", None)],
            ),
            sense("明日", "あす", &["明天"]),
            sense("上", "うえ", &["上面"]),
        ],
    )
    .await;
    let key = WordKey {
        writing: "明日".into(),
        origin: String::new(),
    };
    assert!(delete_word(&db, &key).await.unwrap());
    assert_eq!(all_words(&db).await.len(), 1);
    assert_eq!(all_senses(&db).await.len(), 1);
    assert!(all_sentences(&db).await.is_empty());
    assert!(!delete_word(&db, &key).await.unwrap());
}

#[tokio::test]
async fn a_reading_goes_with_its_senses_and_the_word_stays() {
    let db = memory_db().await;
    merge(
        &db,
        &[
            sense("諦める", "あきらねる", &["放弃"]),
            sense("諦める", "あきらめる", &["死心"]),
        ],
    )
    .await;
    let key = ReadingKey {
        writing: "諦める".into(),
        origin: String::new(),
        kana: "あきらねる".into(),
    };
    assert!(delete_reading(&db, &key).await.unwrap());
    assert_eq!(all_words(&db).await.len(), 1);
    assert_eq!(all_readings(&db).await.len(), 1);
    assert_eq!(all_senses(&db).await[0].primary_gloss, "死心");
    assert!(!delete_reading(&db, &key).await.unwrap());
}

#[tokio::test]
async fn a_sense_goes_and_its_reading_stays() {
    let db = memory_db().await;
    merge(&db, &[sense("明日", "あした", &["明天"])]).await;
    let key = sense_key("明日", "あした", "明天");
    assert!(delete_sense(&db, &key).await.unwrap());
    assert!(all_senses(&db).await.is_empty());
    assert_eq!(all_readings(&db).await.len(), 1);
    assert!(!delete_sense(&db, &key).await.unwrap());
}

#[tokio::test]
async fn a_sentence_is_found_by_its_plain_text() {
    let db = memory_db().await;
    merge(
        &db,
        &[with_sentences(
            sense("明日", "あした", &["明天"]),
            &[
                sentence("[[{明日|あした}]]だ", None),
                sentence("[[{明日|あした}]]です", None),
            ],
        )],
    )
    .await;
    let key = sense_key("明日", "あした", "明天");
    assert!(delete_sentence(&db, &key, "[[明日]]だ").await.unwrap());
    let rest = all_sentences(&db).await;
    assert_eq!(rest.len(), 1);
    assert_eq!(rest[0].marked, "[[{明日|あした}]]です");
    assert!(
        !delete_sentence(&db, &key, "[[{明日|あした}]]だ")
            .await
            .unwrap()
    );
}
