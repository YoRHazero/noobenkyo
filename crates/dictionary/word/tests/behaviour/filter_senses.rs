//! Senses matching a filter, with their progress.

use dictionary_word::SenseKey;
use dictionary_word::store::query::filter_senses::{SenseProgress, filter_senses};
use dictionary_word::store::query::{Completeness, ContentFilter, ProgressState, SenseOrder};
use sea_orm::DatabaseConnection;

use crate::support::{
    memory_db, merge, progress_at, sense, sentence, with_progress, with_sentences,
};

fn with_sentence() -> ContentFilter {
    ContentFilter {
        completeness: vec![Completeness::WithSentence],
        ..ContentFilter::default()
    }
}

fn kana(found: &[SenseProgress]) -> Vec<&str> {
    found.iter().map(|s| s.key.kana.as_str()).collect()
}

/// 語: あ, い, う, え with a sentence each; お without.
async fn fixture(db: &DatabaseConnection) {
    let mut contents: Vec<_> = ["あ", "い", "う", "え"]
        .into_iter()
        .map(|kana| with_sentences(sense("語", kana, &["义"]), &[sentence("[[{語|ご}]]", None)]))
        .collect();
    contents.push(sense("語", "お", &["义"]));
    merge(db, &contents).await;
}

#[tokio::test]
async fn takes_matching_senses_in_creation_order() {
    let db = memory_db().await;
    fixture(&db).await;
    let found = filter_senses(&db, &with_sentence(), &[], SenseOrder::Creation, Some(2))
        .await
        .unwrap();
    assert_eq!(kana(&found), ["あ", "い"]);
    assert_eq!(found[0].progress, progress_at(0, 0, 0));
    let all = filter_senses(&db, &with_sentence(), &[], SenseOrder::Creation, None)
        .await
        .unwrap();
    assert_eq!(all.len(), 4);
}

#[tokio::test]
async fn excluded_senses_do_not_shorten_the_result() {
    let db = memory_db().await;
    fixture(&db).await;
    let exclude: Vec<SenseKey> =
        filter_senses(&db, &with_sentence(), &[], SenseOrder::Creation, Some(2))
            .await
            .unwrap()
            .into_iter()
            .map(|s| s.key)
            .collect();
    let found = filter_senses(&db, &with_sentence(), &exclude, SenseOrder::Random, Some(2))
        .await
        .unwrap();
    let mut got = kana(&found);
    got.sort_unstable();
    assert_eq!(got, ["う", "え"]);
}

#[tokio::test]
async fn filters_by_progress() {
    let db = memory_db().await;
    merge(
        &db,
        &[
            with_progress(
                sense("語", "あ", &["义"]),
                (2, 1, 0),
                "2026-10-01T00:00:00+08:00",
            ),
            sense("語", "い", &["义"]),
        ],
    )
    .await;
    let filter = ContentFilter {
        progress: ProgressState::Untouched,
        ..ContentFilter::default()
    };
    let found = filter_senses(&db, &filter, &[], SenseOrder::Creation, None)
        .await
        .unwrap();
    assert_eq!(kana(&found), ["い"]);
}
