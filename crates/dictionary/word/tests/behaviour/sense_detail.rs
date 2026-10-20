//! One sense in full.

use dictionary_word::SentenceContent;
use dictionary_word::store::query::sense_detail::sense_detail;

use crate::support::{
    memory_db, merge, progress_at, sense, sense_key, sentence, with_labels, with_sentences,
};

#[tokio::test]
async fn loads_everything_about_a_sense() {
    let db = memory_db().await;
    let ashita = with_sentences(
        with_labels(sense("明日", "あした", &["明天"]), &["N5"]),
        &[sentence("[[{明日|あした}]]", Some("明天"))],
    );
    merge(
        &db,
        &[
            ashita,
            sense("明日", "あした", &["将来"]),
            sense("明日", "あす", &["明天"]),
        ],
    )
    .await;

    let detail = sense_detail(&db, &sense_key("明日", "あした", "明天"))
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
        sense_detail(&db, &sense_key("明日", "あした", "明天"))
            .await
            .unwrap()
            .is_none()
    );
}
