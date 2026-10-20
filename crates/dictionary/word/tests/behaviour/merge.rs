//! Merging content into the tables, layer by layer.

use dictionary_word::SentenceContent;
use dictionary_word::store::merge::{MergeOutcome, SenseOutcome};
use dictionary_word::store::query::sense_detail::sense_detail;

use crate::support::{
    all_labels, all_senses, all_words, memory_db, merge, now, sense, sense_key, sentence,
    with_labels, with_progress, with_sentences, word_only,
};

#[tokio::test]
async fn creates_every_layer() {
    let db = memory_db().await;
    let content = with_sentences(
        with_labels(sense("明日", "あした", &["明天"]), &["N5"]),
        &[sentence("[[{明日|あした}]]", None)],
    );
    assert_eq!(
        merge(&db, &[content]).await,
        [MergeOutcome {
            word_added: true,
            reading_added: true,
            sense: SenseOutcome::Added,
            sentences_added: 1,
            progress_updated: false,
        }]
    );
    let detail = sense_detail(&db, &sense_key("明日", "あした", "明天"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(detail.glosses, ["明天"]);
    assert_eq!(detail.progress.modified_at, now());
    assert_eq!(detail.labels, ["N5"]);
}

#[tokio::test]
async fn stops_at_the_given_layer() {
    let db = memory_db().await;
    let outcome = &merge(&db, &[word_only("パソコン", "personal computer", "")]).await[0];
    assert!(outcome.word_added && !outcome.reading_added);
    assert_eq!(all_words(&db).await[0].origin, "personal computer");
}

#[tokio::test]
async fn merges_intersecting_glosses_into_secondary() {
    let db = memory_db().await;
    let first = with_labels(sense("諦める", "あきらめる", &["放弃"]), &["N3"]);
    let second = with_labels(sense("諦める", "あきらめる", &["死心", "放弃"]), &["动词"]);
    let outcomes = merge(&db, &[first, second]).await;
    assert_eq!(outcomes[1].sense, SenseOutcome::Merged);
    assert_eq!(all_senses(&db).await.len(), 1);
    let detail = sense_detail(&db, &sense_key("諦める", "あきらめる", "放弃"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(detail.glosses, ["放弃", "死心"]);
    assert_eq!(all_labels(&db).await.len(), 2);
}

#[tokio::test]
async fn reports_conflict_without_changes() {
    let db = memory_db().await;
    let outcomes = merge(
        &db,
        &[
            sense("上", "うえ", &["上面"]),
            sense("上", "うえ", &["上级"]),
            sense("上", "うえ", &["上面", "上级"]),
        ],
    )
    .await;
    assert_eq!(
        outcomes[2].sense,
        SenseOutcome::Conflict(vec!["上面".into(), "上级".into()])
    );
    assert_eq!(all_senses(&db).await.len(), 2);
}

#[tokio::test]
async fn keeps_existing_sentence_with_same_plain_text() {
    let db = memory_db().await;
    let first = with_sentences(
        sense("明日", "あした", &["明天"]),
        &[sentence("[[{明日|あした}]]だ", Some("旧"))],
    );
    let second = with_sentences(
        sense("明日", "あした", &["明天"]),
        &[
            sentence("[[{明日|あした}]]だ", Some("新")),
            sentence("[[{明日|あした}]]よ", None),
        ],
    );
    let outcomes = merge(&db, &[first, second]).await;
    assert_eq!(outcomes[1].sentences_added, 1);
    let detail = sense_detail(&db, &sense_key("明日", "あした", "明天"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        detail.sentences,
        [
            SentenceContent {
                marked: "[[{明日|あした}]]だ".into(),
                translation: Some("旧".into()),
            },
            SentenceContent {
                marked: "[[{明日|あした}]]よ".into(),
                translation: None,
            },
        ]
    );
}

#[tokio::test]
async fn takes_later_progress() {
    let db = memory_db().await;
    let base = sense("明日", "あした", &["明天"]);
    let outcomes = merge(
        &db,
        &[
            with_progress(base.clone(), (1, 0, 0), "2026-10-01T00:00:00+08:00"),
            with_progress(base.clone(), (5, 0, 0), "2026-09-01T00:00:00+08:00"),
            with_progress(base, (3, 0, 0), "2026-10-02T00:00:00+08:00"),
        ],
    )
    .await;
    assert!(!outcomes[1].progress_updated);
    assert!(outcomes[2].progress_updated);
    let detail = sense_detail(&db, &sense_key("明日", "あした", "明天"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(detail.progress.stage, 3);
}

#[tokio::test]
async fn fills_empty_stratum_only() {
    let db = memory_db().await;
    merge(
        &db,
        &[
            word_only("明日", "", ""),
            word_only("明日", "", "和语"),
            word_only("明日", "", "汉语"),
        ],
    )
    .await;
    assert_eq!(all_words(&db).await[0].stratum, "和语");
}

#[tokio::test]
async fn later_contents_see_earlier_ones() {
    let db = memory_db().await;
    let ashita = sense("明日", "あした", &["明天"]);
    let asu = sense("明日", "あす", &["明天"]);
    let outcomes = merge(&db, &[ashita.clone(), asu, ashita]).await;
    assert!(outcomes[0].word_added);
    assert!(!outcomes[1].word_added && outcomes[1].reading_added);
    assert_eq!(outcomes[2].sense, SenseOutcome::Merged);
    assert_eq!(all_senses(&db).await.len(), 2);
}
