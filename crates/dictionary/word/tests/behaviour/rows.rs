//! Which rows match a filter: senses, and readings or words with nothing below.

use dictionary_word::store::query::{Completeness, ContentFilter, ProgressState, matching_rows};
use sea_orm::DatabaseConnection;

use crate::support::{
    memory_db, merge, reading_only, sense, sentence, with_progress, with_sentences, word_only,
};

/// 明日: あした (sense with a sentence, learned), あす (sense without sentences);
/// 上: うえ (reading only); パソコン (word only, origin "personal computer").
async fn fixture(db: &DatabaseConnection) {
    let ashita = with_sentences(
        sense("明日", "あした", &["明天"]),
        &[sentence("[[{明日|あした}]]", None)],
    );
    merge(
        db,
        &[
            with_progress(ashita, (1, 1, 0), "2026-10-01T00:00:00+08:00"),
            sense("明日", "あす", &["明天"]),
            reading_only("上", "うえ"),
            word_only("パソコン", "personal computer", ""),
        ],
    )
    .await;
}

/// Each row as (word id, has a reading, has a sense).
async fn levels(db: &DatabaseConnection, filter: ContentFilter) -> Vec<(i64, bool, bool)> {
    matching_rows(db, &filter)
        .await
        .unwrap()
        .into_iter()
        .map(|row| {
            (
                row.word_id,
                row.reading_id.is_some(),
                row.sense_id.is_some(),
            )
        })
        .collect()
}

#[tokio::test]
async fn lists_every_row_word_by_word() {
    let db = memory_db().await;
    fixture(&db).await;
    assert_eq!(
        levels(&db, ContentFilter::default()).await,
        [
            (1, true, true),
            (1, true, true),
            (2, true, false),
            (3, false, false)
        ]
    );
}

#[tokio::test]
async fn filters_by_completeness() {
    let db = memory_db().await;
    fixture(&db).await;
    let only = |levels: Vec<Completeness>| ContentFilter {
        completeness: levels,
        ..ContentFilter::default()
    };
    assert_eq!(
        levels(&db, only(vec![Completeness::WithSentence]))
            .await
            .len(),
        1
    );
    assert_eq!(
        levels(&db, only(vec![Completeness::NoSentence])).await,
        [(1, true, true)]
    );
    assert_eq!(
        levels(
            &db,
            only(vec![Completeness::WordOnly, Completeness::ReadingOnly])
        )
        .await,
        [(2, true, false), (3, false, false)]
    );
}

#[tokio::test]
async fn progress_keeps_senses_only() {
    let db = memory_db().await;
    fixture(&db).await;
    let progress = |state| ContentFilter {
        progress: state,
        ..ContentFilter::default()
    };
    assert_eq!(
        levels(&db, progress(ProgressState::Untouched)).await.len(),
        1
    );
    assert_eq!(levels(&db, progress(ProgressState::Touched)).await.len(), 1);
}

#[tokio::test]
async fn text_matches_each_layer() {
    let db = memory_db().await;
    fixture(&db).await;
    let text = |text: &str| ContentFilter {
        text: Some(text.into()),
        ..ContentFilter::default()
    };
    assert_eq!(levels(&db, text("明天")).await.len(), 2);
    assert_eq!(levels(&db, text("アス")).await, [(1, true, true)]);
    assert_eq!(levels(&db, text("うえ")).await, [(2, true, false)]);
    assert_eq!(levels(&db, text("computer")).await, [(3, false, false)]);
    assert_eq!(levels(&db, text(" ")).await.len(), 4);
}
