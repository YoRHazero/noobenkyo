//! A page of rows for the dictionary page.

use dictionary_word::WordKey;
use dictionary_word::store::query::ContentFilter;
use dictionary_word::store::query::search::{SearchRow, search};

use crate::support::{memory_db, merge, sense, sentence, with_labels, with_sentences, word_only};

#[tokio::test]
async fn pages_rows_with_their_summaries() {
    let db = memory_db().await;
    let ashita = with_sentences(
        with_labels(sense("明日", "あした", &["明天", "翌日"]), &["N5"]),
        &[sentence("[[{明日|あした}]]", None)],
    );
    merge(&db, &[ashita, word_only("パソコン", "", "")]).await;

    let page = search(&db, &ContentFilter::default(), 0, 10).await.unwrap();
    assert_eq!(page.total, 2);
    let summary = page.rows[0].sense.as_ref().unwrap();
    assert_eq!(page.rows[0].kana.as_deref(), Some("あした"));
    assert_eq!(summary.primary_gloss, "明天");
    assert_eq!(summary.secondary_glosses, ["翌日"]);
    assert_eq!(summary.labels, ["N5"]);
    assert_eq!(summary.sentence_count, 1);
    assert_eq!(
        page.rows[1],
        SearchRow {
            word: WordKey {
                writing: "パソコン".into(),
                origin: String::new()
            },
            kana: None,
            sense: None,
        }
    );

    let second = search(&db, &ContentFilter::default(), 1, 10).await.unwrap();
    assert_eq!(second.total, 2);
    assert_eq!(second.rows.len(), 1);
}
