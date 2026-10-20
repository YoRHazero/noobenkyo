//! Changing a sense in place: its content, or its progress.

use dictionary_word::store::edit::replace_sense::replace_sense;
use dictionary_word::store::edit::update_progress::update_progress;
use dictionary_word::store::query::sense_detail::sense_detail;
use dictionary_word::{Replaced, SenseKey, SensePatch, SentenceContent};
use sea_orm::DatabaseConnection;

use crate::support::{
    memory_db, merge, progress_at, sense, sense_key, sentence, with_labels, with_sentences,
};

fn key(gloss: &str) -> SenseKey {
    sense_key("諦める", "あきらめる", gloss)
}

/// 諦める: 放弃 (label N3, one sentence) and 死心.
async fn fixture(db: &DatabaseConnection) {
    let giving_up = with_sentences(
        with_labels(sense("諦める", "あきらめる", &["放弃"]), &["N3"]),
        &[sentence("[[{諦|あきら}めた]]", None)],
    );
    merge(db, &[giving_up, sense("諦める", "あきらめる", &["死心"])]).await;
}

#[tokio::test]
async fn replaces_given_fields_and_keeps_the_rest() {
    let db = memory_db().await;
    fixture(&db).await;
    let patch = SensePatch {
        glosses: Some(vec!["放弃（做某事）".into(), "放弃".into()]),
        sentences: Some(vec![SentenceContent {
            marked: "{夢|ゆめ}を[[{諦|あきら}めた]]".into(),
            translation: Some("放弃了梦想".into()),
        }]),
        ..SensePatch::default()
    };
    let replaced = replace_sense(&db, &key("放弃"), &patch).await.unwrap();
    assert_eq!(
        replaced,
        Replaced::Done {
            key: key("放弃（做某事）")
        }
    );
    let detail = sense_detail(&db, &key("放弃（做某事）"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(detail.glosses, ["放弃（做某事）", "放弃"]);
    assert_eq!(detail.labels, ["N3"]);
    assert_eq!(detail.sentences.len(), 1);
    assert_eq!(
        detail.sentences[0].translation.as_deref(),
        Some("放弃了梦想")
    );
}

#[tokio::test]
async fn rejects_without_changing_anything() {
    let db = memory_db().await;
    fixture(&db).await;
    let patch = SensePatch {
        glosses: Some(vec!["死心".into()]),
        labels: Some(vec![]),
        ..SensePatch::default()
    };
    let replaced = replace_sense(&db, &key("放弃"), &patch).await.unwrap();
    assert!(matches!(replaced, Replaced::Rejected { .. }));
    let detail = sense_detail(&db, &key("放弃")).await.unwrap().unwrap();
    assert_eq!(detail.labels, ["N3"]);
}

#[tokio::test]
async fn reports_a_missing_sense() {
    let db = memory_db().await;
    let replaced = replace_sense(&db, &key("放弃"), &SensePatch::default())
        .await
        .unwrap();
    assert_eq!(replaced, Replaced::NotFound);
}

#[tokio::test]
async fn overwrites_progress() {
    let db = memory_db().await;
    fixture(&db).await;
    assert!(
        update_progress(&db, &key("死心"), &progress_at(2, 3, 1))
            .await
            .unwrap()
    );
    let detail = sense_detail(&db, &key("死心")).await.unwrap().unwrap();
    assert_eq!(detail.progress, progress_at(2, 3, 1));
}

#[tokio::test]
async fn progress_of_a_missing_sense_is_not_written() {
    let db = memory_db().await;
    assert!(
        !update_progress(&db, &key("死心"), &progress_at(2, 3, 1))
            .await
            .unwrap()
    );
}
