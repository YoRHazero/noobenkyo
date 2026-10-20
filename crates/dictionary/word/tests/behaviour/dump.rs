//! Rows as content, for export.

use dictionary_word::store::query::dump::dump;
use dictionary_word::store::query::{Completeness, ContentFilter};
use dictionary_word::{Content, ContentProgress, ReadingContent, SenseContent, SentenceContent};

use crate::support::{
    memory_db, merge, now, reading_only, sense, sentence, with_labels, with_sentences, word_only,
};

#[tokio::test]
async fn gives_each_row_once_word_by_word() {
    let db = memory_db().await;
    let ashita = with_sentences(
        with_labels(sense("明日", "あした", &["明天", "翌日"]), &["N5"]),
        &[sentence("[[{明日|あした}]]", Some("明天"))],
    );
    merge(
        &db,
        &[
            ashita,
            word_only("パソコン", "personal computer", "外来语"),
            reading_only("上", "うえ"),
        ],
    )
    .await;

    assert_eq!(
        dump(&db, &ContentFilter::default(), None).await.unwrap(),
        [
            Content {
                writing: "明日".into(),
                origin: String::new(),
                stratum: String::new(),
                reading: Some(ReadingContent {
                    kana: "あした".into(),
                    sense: Some(SenseContent {
                        glosses: vec!["明天".into(), "翌日".into()],
                        labels: vec!["N5".into()],
                        sentences: vec![SentenceContent {
                            marked: "[[{明日|あした}]]".into(),
                            translation: Some("明天".into()),
                        }],
                        progress: Some(ContentProgress {
                            stage: 0,
                            passes: 0,
                            lapses: 0,
                            modified_at: Some(now()),
                        }),
                    }),
                }),
            },
            Content {
                writing: "パソコン".into(),
                origin: "personal computer".into(),
                stratum: "外来语".into(),
                reading: None,
            },
            Content {
                writing: "上".into(),
                origin: String::new(),
                stratum: String::new(),
                reading: Some(ReadingContent {
                    kana: "うえ".into(),
                    sense: None,
                }),
            },
        ]
    );
}

#[tokio::test]
async fn takes_only_matching_rows_up_to_the_limit() {
    let db = memory_db().await;
    merge(
        &db,
        &[
            sense("語", "あ", &["义"]),
            sense("語", "い", &["义"]),
            sense("語", "う", &["义"]),
            with_sentences(sense("語", "え", &["义"]), &[sentence("[[{語|ご}]]", None)]),
        ],
    )
    .await;
    let filter = ContentFilter {
        completeness: vec![Completeness::NoSentence],
        ..ContentFilter::default()
    };
    let kana: Vec<String> = dump(&db, &filter, Some(2))
        .await
        .unwrap()
        .into_iter()
        .map(|c| c.reading.unwrap().kana)
        .collect();
    assert_eq!(kana, ["あ", "い"]);
}
