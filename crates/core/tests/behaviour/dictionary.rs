//! The dictionary's commands: entry texts in and out, and sentences
//! rendered for the frontend.

use dictionary::{ContentFilter, Fragment, ImportOptions, SenseKey, SentenceDisplay, TargetPiece};
use noobenkyo_core::commands::dictionary::entries::{
    dictionary_check_entries, dictionary_default_import_options, dictionary_export,
    dictionary_preview_entries,
};
use noobenkyo_core::commands::dictionary::query::dictionary_sense_detail;
use noobenkyo_core::commands::dictionary::render::dictionary_render_sentence;
use noobenkyo_core::commands::stats::dictionary::dictionary_stats;

use crate::support::{SAMPLE, TestApp};

#[tokio::test]
async fn importing_takes_the_text() {
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
async fn the_preview_shows_what_importing_merges() {
    let app = TestApp::fresh().await;
    let options = dictionary_default_import_options();
    let preview = dictionary_preview_entries(SAMPLE.to_owned(), options).await;
    assert_eq!(
        (
            preview.lines,
            preview.entries.len(),
            preview.skipped_katakana.len()
        ),
        (12, 10, 2)
    );
    assert!(preview.problems.is_empty());
    let report = app.import_sample().await;
    assert_eq!(report.senses_added, 10);
}

#[tokio::test]
async fn checking_follows_the_options() {
    let inflected = r#"{"writing": "食べた", "kana": "たべた", "glosses": ["吃了"]}"#;
    let lenient = ImportOptions {
        detect_inflections: false,
        ..ImportOptions::default()
    };
    let strict = ImportOptions {
        detect_inflections: true,
        ..ImportOptions::default()
    };
    assert!(
        dictionary_check_entries(inflected.to_owned(), lenient)
            .await
            .is_empty()
    );
    assert_eq!(
        dictionary_check_entries(inflected.to_owned(), strict).await,
        ["第 1 行 食べた：疑似活用形"]
    );
    // A skipped line is not a warning.
    assert!(
        dictionary_check_entries("not json".to_owned(), lenient)
            .await
            .is_empty()
    );
}

#[tokio::test]
async fn exporting_gives_the_text() {
    let app = TestApp::with_sample().await;
    let exported = dictionary_export(app.state(), ContentFilter::default(), None, true)
        .await
        .unwrap();
    assert_eq!(exported.entries, 10);
    assert_eq!(exported.text.lines().count(), 10);
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
