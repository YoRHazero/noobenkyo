//! The entry tools read a text as a generated list.

use ai_tools::{ExistingWord, check_entries, existing_entries};
use dictionary::{ImportOptions, WordKey};

use crate::support::dictionary_with;

#[test]
fn the_check_looks_at_katakana_words_too() {
    // Outside lists skip pure katakana words, so their warnings never show.
    let text = r#"{"writing": "パソコン", "colour": "灰"}"#;
    assert!(dictionary::check_entries(text, ImportOptions::default()).is_empty());
    assert_eq!(check_entries(text), ["第 1 行 パソコン：未知字段 colour"]);
}

#[test]
fn the_check_leaves_out_lines_dropped_whole() {
    let text = concat!(
        r#"{"writing": "上", "kana": "うえ", "glosses": ["上面"], "colour": "红"}"#,
        "\n",
        r#"{"writing": "Wi-Fi", "colour": "红"}"#,
        "\n",
        "not json",
    );
    assert_eq!(check_entries(text), ["第 1 行 上：未知字段 colour"]);
}

#[tokio::test]
async fn lists_the_lines_whose_word_is_already_there() {
    let dir = tempfile::tempdir().unwrap();
    let dictionary = dictionary_with(
        &dir,
        r#"{"writing": "上", "kana": "うえ", "glosses": ["上面"]}"#,
    )
    .await;
    let text = concat!(
        r#"{"writing": "下"}"#,
        "\n",
        r#"{"writing": "上", "kana": "かみ", "glosses": ["上游"]}"#,
        "\n",
        r#"{"writing": "パソコン"}"#,
        "\n",
        r#"{"writing": ""}"#,
    );
    assert_eq!(
        existing_entries(&dictionary, text).await.unwrap(),
        [ExistingWord {
            line: 2,
            word: WordKey {
                writing: "上".into(),
                origin: String::new(),
            },
        }]
    );
}
