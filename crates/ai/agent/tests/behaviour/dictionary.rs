//! `noobenkyo dictionary …`

use ai_tools::{Delivery, Inbox};

use crate::support::{dictionary_with, file, noobenkyo};

const SENSE: &str = r#"{"writing": "上", "kana": "うえ", "glosses": ["上面"]}"#;

#[test]
fn the_guide_needs_no_data() {
    let dir = tempfile::tempdir().unwrap();
    let run = noobenkyo(&dir.path().join("none"), &["dictionary", "guide"], None);
    assert_eq!(run.code, 0);
    assert!(!run.stdout.is_empty());
}

#[test]
fn check_is_silent_when_the_text_passes_and_exits_1_otherwise() {
    let dir = tempfile::tempdir().unwrap();
    let good = file(dir.path(), "good.jsonl", SENSE);
    let run = noobenkyo(dir.path(), &["dictionary", "check", &good], None);
    assert_eq!((run.code, run.stdout.as_str()), (0, ""));

    let bad = r#"{"writing": "上", "kana": "うえ", "glosses": ["上面"], "colour": "红"}"#;
    let run = noobenkyo(dir.path(), &["dictionary", "check", "-"], Some(bad));
    assert_eq!(run.code, 1);
    assert_eq!(run.stdout, "第 1 行 上：未知字段 colour\n");
}

#[tokio::test]
async fn export_leaves_out_progress() {
    let dir = tempfile::tempdir().unwrap();
    let with_progress = r#"{"writing": "上", "kana": "うえ", "glosses": ["上面"], "stage": 2, "passes": 3, "lapses": 0}"#;
    dictionary_with(dir.path(), with_progress).await;
    let run = noobenkyo(
        dir.path(),
        &["dictionary", "export", "--completeness", "no-sentence"],
        None,
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        run.stdout,
        "{\"writing\":\"上\",\"kana\":\"うえ\",\"glosses\":[\"上面\"]}\n"
    );
}

#[tokio::test]
async fn existing_names_the_lines_already_in_the_dictionary() {
    let dir = tempfile::tempdir().unwrap();
    dictionary_with(dir.path(), SENSE).await;
    let text = "{\"writing\": \"下\"}\n{\"writing\": \"上\"}\n";
    let run = noobenkyo(dir.path(), &["dictionary", "existing", "-"], Some(text));
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(run.stdout, "第 2 行：上\n");
}

#[tokio::test]
async fn send_leaves_the_entries_in_the_inbox() {
    let dir = tempfile::tempdir().unwrap();
    dictionary_with(dir.path(), SENSE).await;
    let words = file(dir.path(), "words.jsonl", SENSE);
    let run = noobenkyo(dir.path(), &["dictionary", "send", &words], None);
    assert_eq!(
        (run.code, run.stdout.as_str()),
        (0, "已送进 inbox：words\n"),
        "{}",
        run.stderr
    );
    assert_eq!(
        Inbox::in_data_dir(dir.path()).read("words").unwrap(),
        Some(Delivery::Entries { text: SENSE.into() })
    );

    let again = noobenkyo(dir.path(), &["dictionary", "send", &words], None);
    assert_eq!(
        (again.code, again.stderr.as_str()),
        (2, "inbox 里已经有 words\n")
    );
    let stdin = noobenkyo(dir.path(), &["dictionary", "send", "-"], Some(SENSE));
    assert_eq!(stdin.code, 2);
}

#[test]
fn reading_data_the_app_never_made_fails_with_exit_2() {
    let dir = tempfile::tempdir().unwrap();
    let run = noobenkyo(dir.path(), &["dictionary", "export"], None);
    assert_eq!(run.code, 2);
    assert!(run.stderr.contains("先打开一次 app"), "{}", run.stderr);
    assert!(!dir.path().join("dictionary").exists());
}
