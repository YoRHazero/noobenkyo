//! Entries in and out: previewing, checking, importing and exporting text.

use dictionary::{
    Completeness, ContentFilter, DictionaryService, Dropped, ImportOptions, WordKey, check_entries,
    generated_import_options, preview_entries,
};

const TEXT: &str = concat!(
    r#"{"writing":"諦める","kana":"あきらめる","glosses":["放弃","死心"],"labels":["N3"],"sentences":[{"marked":"[[{諦|あきら}めない]]で","translation":"别放弃"}],"stage":2,"passes":3,"lapses":1,"modified_at":"2026-10-17T21:05:02.117+08:00"}"#,
    "\n",
    r#"{"writing":"上","kana":"うえ","glosses":["上面"]}"#,
    "\n",
    r#"{"writing":"パソコン","origin":"personal computer","origin_language":"英语"}"#,
    "\n",
);

async fn service(dir: &tempfile::TempDir) -> DictionaryService {
    DictionaryService::open(dir.path()).await.unwrap()
}

fn keep_katakana() -> ImportOptions {
    ImportOptions {
        drop_pure_katakana: false,
        ..ImportOptions::default()
    }
}

#[tokio::test]
async fn export_gives_back_what_was_imported() {
    let dir = tempfile::tempdir().unwrap();
    let dictionary = service(&dir).await;
    let report = dictionary.import(TEXT, keep_katakana()).await.unwrap();
    assert_eq!(
        (report.lines, report.words_added, report.senses_added),
        (3, 3, 2)
    );
    assert!(report.problems.is_empty());

    let exported = dictionary
        .export(&ContentFilter::default(), None, true)
        .await
        .unwrap();
    let lines: Vec<&str> = exported.text.lines().collect();
    let source: Vec<&str> = TEXT.lines().collect();
    assert_eq!((lines[0], lines[2]), (source[0], source[2]));
    assert!(lines[1].starts_with(r#"{"writing":"上","kana":"うえ","glosses":["上面"],"stage":0"#));
    assert_eq!(exported.entries, 3);
}

#[tokio::test]
async fn exports_only_matching_rows() {
    let dir = tempfile::tempdir().unwrap();
    let dictionary = service(&dir).await;
    dictionary.import(TEXT, keep_katakana()).await.unwrap();
    let filter = ContentFilter {
        completeness: vec![Completeness::NoSentence],
        ..ContentFilter::default()
    };
    let exported = dictionary.export(&filter, Some(20), false).await.unwrap();
    assert_eq!(
        exported.text,
        "{\"writing\":\"上\",\"kana\":\"うえ\",\"glosses\":[\"上面\"]}\n"
    );
}

#[tokio::test]
async fn import_reports_what_check_reports() {
    let dir = tempfile::tempdir().unwrap();
    let dictionary = service(&dir).await;
    let text = r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"], "sentences": [{"marked": "[[明日]]"}]}"#;
    let report = dictionary
        .import(text, ImportOptions::default())
        .await
        .unwrap();
    assert_eq!(report.senses_added, 1);
    assert_eq!(report.problems.len(), 1);
    assert_eq!(check_entries(text, ImportOptions::default()).len(), 1);
}

#[tokio::test]
async fn import_merges_exactly_the_preview_entries() {
    let dir = tempfile::tempdir().unwrap();
    let dictionary = service(&dir).await;
    let options = ImportOptions {
        detect_inflections: true,
        ..ImportOptions::default()
    };
    let preview = preview_entries(TEXT, options);
    let writings: Vec<&str> = preview
        .entries
        .iter()
        .map(|entry| entry.content.writing.as_str())
        .collect();
    assert_eq!(writings, ["諦める", "上"]);
    assert_eq!(preview.skipped_katakana.len(), 1);

    let report = dictionary.import(TEXT, options).await.unwrap();
    assert_eq!(report.skipped_katakana, 1);
    let exported = dictionary
        .export(&ContentFilter::default(), None, false)
        .await
        .unwrap();
    assert_eq!(exported.entries, 2);
}

#[tokio::test]
async fn the_report_adds_only_merge_problems_to_the_preview() {
    let dir = tempfile::tempdir().unwrap();
    let dictionary = service(&dir).await;
    let existing = concat!(
        r#"{"writing": "上", "kana": "うえ", "glosses": ["上面"]}"#,
        "\n",
        r#"{"writing": "上", "kana": "うえ", "glosses": ["上级"]}"#,
    );
    dictionary
        .import(existing, ImportOptions::default())
        .await
        .unwrap();

    let text = concat!(
        r#"{"writing": "上", "kana": "うえ", "glosses": ["上面", "上级"]}"#,
        "\n",
        r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"], "sentences": [{"marked": "[[明日]]"}]}"#,
    );
    let preview = preview_entries(text, ImportOptions::default());
    let report = dictionary
        .import(text, ImportOptions::default())
        .await
        .unwrap();
    let (merging, converting): (Vec<_>, Vec<_>) = report
        .problems
        .into_iter()
        .partition(|problem| problem.dropped == Dropped::Sense);
    assert_eq!(converting, preview.problems);
    assert_eq!(merging.len(), 1);
    assert_eq!(merging[0].line, 1);
}

#[tokio::test]
async fn generated_lists_keep_katakana_and_never_set_progress() {
    let dir = tempfile::tempdir().unwrap();
    let dictionary = service(&dir).await;
    let report = dictionary
        .import(TEXT, generated_import_options())
        .await
        .unwrap();
    assert_eq!(report.words_added, 3);
    let exported = dictionary
        .export(&ContentFilter::default(), None, true)
        .await
        .unwrap();
    assert!(
        exported
            .text
            .lines()
            .next()
            .unwrap()
            .contains(r#""stage":0"#)
    );
}

#[tokio::test]
async fn finds_which_words_are_already_there() {
    let dir = tempfile::tempdir().unwrap();
    let dictionary = service(&dir).await;
    dictionary.import(TEXT, keep_katakana()).await.unwrap();
    let key = |writing: &str, origin: &str| WordKey {
        writing: writing.into(),
        origin: origin.into(),
    };
    let found = dictionary
        .existing_words(&[
            key("下", ""),
            key("パソコン", "personal computer"),
            key("上", ""),
        ])
        .await
        .unwrap();
    assert_eq!(found, [key("パソコン", "personal computer"), key("上", "")]);
}
