//! The dictionary's commands: files in and out, and sentences rendered for
//! the frontend.

use dictionary::{ContentFilter, Fragment, ImportOptions, SenseKey, SentenceDisplay, TargetPiece};
use noobenkyo_lib::commands::dictionary::entries::{
    dictionary_check_entries, dictionary_default_import_options, dictionary_export,
    dictionary_import,
};
use noobenkyo_lib::commands::dictionary::query::dictionary_sense_detail;
use noobenkyo_lib::commands::dictionary::render::dictionary_render_sentence;
use noobenkyo_lib::commands::stats::dictionary::dictionary_stats;

use crate::support::TestApp;

#[tokio::test]
async fn importing_reads_the_picked_file() {
    let app = TestApp::fresh().await;
    assert_eq!(
        dictionary_default_import_options(),
        ImportOptions::default()
    );
    let report = app.import_sample().await;
    assert_eq!((report.lines, report.skipped_katakana), (12, 2));
    let stats = dictionary_stats(app.state()).await.unwrap();
    assert_eq!((stats.with_sentence, stats.no_sentence), (9, 1));
    assert_eq!(stats.untouched, 9);
}

#[tokio::test]
async fn a_file_that_cannot_be_read_is_an_error() {
    let app = TestApp::fresh().await;
    let missing = app.user_file("missing.jsonl", "");
    std::fs::remove_file(&missing).unwrap();
    let error = dictionary_import(app.state(), missing, ImportOptions::default())
        .await
        .unwrap_err();
    assert!(
        error.message.starts_with("读取文件失败："),
        "{}",
        error.message
    );
}

#[tokio::test]
async fn checking_reads_the_picked_file() {
    let app = TestApp::fresh().await;
    let good = app.user_file("good.jsonl", r#"{"writing": "上", "kana": "うえ"}"#);
    let bad = app.user_file("bad.jsonl", "not json");
    assert!(dictionary_check_entries(good).await.unwrap().is_empty());
    assert_eq!(dictionary_check_entries(bad).await.unwrap().len(), 1);
}

#[tokio::test]
async fn exporting_writes_the_picked_file() {
    let app = TestApp::with_sample().await;
    let path = app.user_file("export.jsonl", "");
    let entries = dictionary_export(
        app.state(),
        path.clone(),
        ContentFilter::default(),
        None,
        true,
    )
    .await
    .unwrap();
    assert_eq!(entries, 10);
    assert_eq!(std::fs::read_to_string(path).unwrap().lines().count(), 10);
}

#[tokio::test]
async fn sense_detail_renders_every_sentence_in_full() {
    let app = TestApp::with_sample().await;
    let key = SenseKey {
        writing: "諦める".into(),
        origin: String::new(),
        kana: "あきらめる".into(),
        primary_gloss: "放弃".into(),
    };
    let payload = dictionary_sense_detail(app.state(), key)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(payload.sentences.len(), payload.detail.sentences.len());
    let rendered = dictionary_render_sentence(
        payload.detail.sentences[1].marked.clone(),
        SentenceDisplay::Full,
    )
    .unwrap();
    assert_eq!(payload.sentences[1], rendered);
    assert!(rendered.contains(&Fragment::Target {
        pieces: vec![
            TargetPiece::Ruby {
                base: "諦".into(),
                ruby: "あきら".into()
            },
            TargetPiece::Text {
                text: "めた".into()
            }
        ]
    }));
}

#[tokio::test]
async fn a_gone_sense_has_no_detail() {
    let app = TestApp::fresh().await;
    let key = SenseKey {
        writing: "諦める".into(),
        origin: String::new(),
        kana: "あきらめる".into(),
        primary_gloss: "放弃".into(),
    };
    assert!(
        dictionary_sense_detail(app.state(), key)
            .await
            .unwrap()
            .is_none()
    );
}
