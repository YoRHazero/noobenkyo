//! Which words are already in the dictionary.

use dictionary_word::WordKey;
use dictionary_word::store::query::existing_words::existing_words;

use crate::support::{memory_db, merge, sense, word_only};

fn key(writing: &str, origin: &str) -> WordKey {
    WordKey {
        writing: writing.into(),
        origin: origin.into(),
    }
}

#[tokio::test]
async fn keeps_only_stored_words_in_the_given_order() {
    let db = memory_db().await;
    merge(
        &db,
        &[
            sense("明日", "あした", &["明天"]),
            word_only("上手", "", ""),
        ],
    )
    .await;
    let found = existing_words(&db, &[key("上手", ""), key("諦める", ""), key("明日", "")])
        .await
        .unwrap();
    assert_eq!(found, [key("上手", ""), key("明日", "")]);
}

#[tokio::test]
async fn matches_the_origin_exactly() {
    let db = memory_db().await;
    merge(&db, &[word_only("コーヒー", "koffie", "")]).await;
    let found = existing_words(&db, &[key("コーヒー", ""), key("コーヒー", "koffie")])
        .await
        .unwrap();
    assert_eq!(found, [key("コーヒー", "koffie")]);
}

#[tokio::test]
async fn lists_a_repeated_word_once() {
    let db = memory_db().await;
    merge(&db, &[word_only("上手", "", "")]).await;
    let found = existing_words(&db, &[key("上手", ""), key("上手", "")])
        .await
        .unwrap();
    assert_eq!(found, [key("上手", "")]);
}
