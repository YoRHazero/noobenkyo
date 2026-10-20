//! Counts by layer and by progress.

use dictionary_word::store::query::stats::{DictionaryStats, StageCount, count_stats};

use crate::support::{
    memory_db, merge, reading_only, sense, sentence, with_progress, with_sentences, word_only,
};

const EARLIER: &str = "2026-10-01T00:00:00+08:00";

#[tokio::test]
async fn counts_rows_by_level() {
    let db = memory_db().await;
    merge(
        &db,
        &[
            with_sentences(
                sense("明日", "あした", &["明天"]),
                &[sentence("[[{明日|あした}]]", None)],
            ),
            with_progress(sense("明日", "あす", &["明天"]), (1, 1, 0), EARLIER),
            reading_only("上", "うえ"),
            word_only("パソコン", "", ""),
        ],
    )
    .await;
    assert_eq!(
        count_stats(&db).await.unwrap(),
        DictionaryStats {
            words: 3,
            word_only: 1,
            reading_only: 1,
            no_sentence: 1,
            with_sentence: 1,
            untouched: 1,
            stages: Vec::new(),
        }
    );
}

#[tokio::test]
async fn counts_senses_with_sentences_by_progress() {
    let db = memory_db().await;
    let with_sentence =
        |kana| with_sentences(sense("語", kana, &["义"]), &[sentence("[[{語|ご}]]", None)]);
    merge(
        &db,
        &[
            with_sentence("あ"),
            // Stage 0 after a lapse still counts as touched.
            with_progress(with_sentence("い"), (0, 1, 1), EARLIER),
            with_progress(with_sentence("う"), (2, 1, 0), EARLIER),
            with_progress(with_sentence("え"), (2, 3, 0), EARLIER),
            // Without sentences: counted in neither.
            with_progress(sense("語", "お", &["义"]), (5, 1, 0), EARLIER),
            sense("語", "か", &["义"]),
        ],
    )
    .await;
    let stats = count_stats(&db).await.unwrap();
    assert_eq!(stats.untouched, 1);
    assert_eq!(
        stats.stages,
        [
            StageCount {
                stage: 0,
                senses: 1
            },
            StageCount {
                stage: 2,
                senses: 2
            }
        ]
    );
}
