//! `noobenkyo listening …`

use ai_tools::{Delivery, Inbox, ScriptTarget};
use serde_json::Value;

use crate::support::{SAMPLE_SCRIPT, file, noobenkyo, sample_episode, seeded_listening};

fn json(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

#[tokio::test]
async fn worlds_and_episodes_come_out_as_json() {
    let dir = tempfile::tempdir().unwrap();
    let listening = seeded_listening(dir.path()).await;
    let episode = sample_episode(&listening).await;

    let run = noobenkyo(dir.path(), &["listening", "worlds"], None);
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(json(&run.stdout)[0]["name"], "星见町");

    let run = noobenkyo(dir.path(), &["listening", "episodes", "星见町"], None);
    assert_eq!(run.code, 0, "{}", run.stderr);
    let listed = &json(&run.stdout)[0];
    assert_eq!(listed["uuid"], episode.to_string());
    assert_eq!(
        listed["summary"],
        "美咲在海边的便利店遇到同学健二，两人约好放学后一起去看灯塔。"
    );
    assert!(listed["char_count"].as_u64().unwrap() > 0);
}

#[tokio::test]
async fn context_carries_the_chosen_predecessors_in_full() {
    let dir = tempfile::tempdir().unwrap();
    let listening = seeded_listening(dir.path()).await;
    let episode = sample_episode(&listening).await.to_string();

    let run = noobenkyo(
        dir.path(),
        &["listening", "context", "星见町", "--after", &episode],
        None,
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    let context = json(&run.stdout);
    assert_eq!(context["mains"].as_array().unwrap().len(), 2);
    assert_eq!(
        context["predecessors"][0]["lines"]
            .as_array()
            .unwrap()
            .len(),
        8
    );
    assert_eq!(context["appending"], Value::Null);
}

#[test]
fn check_reads_standard_input() {
    let dir = tempfile::tempdir().unwrap();
    let run = noobenkyo(
        dir.path(),
        &["listening", "check", "-"],
        Some(SAMPLE_SCRIPT),
    );
    assert_eq!((run.code, run.stdout.as_str()), (0, ""));

    let run = noobenkyo(dir.path(), &["listening", "check", "-"], Some("{}"));
    assert_eq!(run.code, 1);
    assert_eq!(run.stdout, "缺少 title\n缺少 summary\n缺少 lines\n");
}

#[tokio::test]
async fn send_checks_first_and_then_fills_the_inbox() {
    let dir = tempfile::tempdir().unwrap();
    seeded_listening(dir.path()).await;
    let inbox = Inbox::in_data_dir(dir.path());
    let ep = file(dir.path(), "ep.json", SAMPLE_SCRIPT);

    let run = noobenkyo(
        dir.path(),
        &["listening", "send", &ep, "--world", "火星"],
        None,
    );
    assert_eq!((run.code, run.stdout.as_str()), (1, "世界 火星 不存在\n"));
    assert!(inbox.names().unwrap().is_empty());

    let run = noobenkyo(
        dir.path(),
        &["listening", "send", &ep, "--world", "星见町"],
        None,
    );
    assert_eq!(
        (run.code, run.stdout.as_str()),
        (0, "已送进 inbox：ep\n"),
        "{}",
        run.stderr
    );
    assert_eq!(
        inbox.read("ep").unwrap(),
        Some(Delivery::Script {
            target: ScriptTarget::NewEpisode {
                world: "星见町".into()
            },
            text: SAMPLE_SCRIPT.into(),
        })
    );
}

#[test]
fn send_needs_exactly_one_target() {
    let dir = tempfile::tempdir().unwrap();
    let run = noobenkyo(dir.path(), &["listening", "send", "-"], Some(SAMPLE_SCRIPT));
    assert_eq!(run.code, 2);
}

/// The agent reads back the script with the user's edits, changes it, and
/// replaces the same delivery.
#[tokio::test]
async fn show_and_replace_work_on_the_same_delivery() {
    let dir = tempfile::tempdir().unwrap();
    seeded_listening(dir.path()).await;
    let inbox = Inbox::in_data_dir(dir.path());
    let ep = file(dir.path(), "ep.json", SAMPLE_SCRIPT);
    let send = |extra: &[&str]| {
        let mut args = vec!["listening", "send", ep.as_str(), "--world", "星见町"];
        args.extend_from_slice(extra);
        noobenkyo(dir.path(), &args, None)
    };

    let replaced_too_early = send(&["--replace"]);
    assert_eq!(
        (replaced_too_early.code, replaced_too_early.stderr.as_str()),
        (2, "inbox 里没有 ep\n")
    );
    assert_eq!(send(&[]).code, 0);
    let again = send(&[]);
    assert_eq!(
        (again.code, again.stderr.as_str()),
        (2, "inbox 里已经有 ep\n")
    );

    // The user edits the title in the preview.
    let edited = SAMPLE_SCRIPT.replace("海边的早晨", "下雨的早晨");
    inbox
        .replace(
            "ep",
            &Delivery::Script {
                target: ScriptTarget::NewEpisode {
                    world: "星见町".into(),
                },
                text: edited.clone(),
            },
        )
        .unwrap();
    let shown = noobenkyo(dir.path(), &["listening", "show", "ep"], None);
    assert_eq!(
        (shown.code, shown.stdout.as_str()),
        (0, format!("{}\n", edited.trim_end()).as_str())
    );

    std::fs::write(&ep, shown.stdout.replace("约好", "说好")).unwrap();
    let replaced = send(&["--replace"]);
    assert_eq!(
        (replaced.code, replaced.stdout.as_str()),
        (0, "已覆写 inbox 里的 ep\n"),
        "{}",
        replaced.stderr
    );
    let Some(Delivery::Script { text, .. }) = inbox.read("ep").unwrap() else {
        panic!("ep is still a script");
    };
    assert!(
        text.contains("下雨的早晨") && text.contains("说好"),
        "{text}"
    );
}

#[test]
fn show_needs_a_waiting_script() {
    let dir = tempfile::tempdir().unwrap();
    let inbox = Inbox::in_data_dir(dir.path());
    let missing = noobenkyo(dir.path(), &["listening", "show", "ep"], None);
    assert_eq!(
        (missing.code, missing.stderr.as_str()),
        (2, "inbox 里没有 ep\n")
    );

    inbox
        .put("words", &Delivery::Entries { text: "{}".into() })
        .unwrap();
    let entries = noobenkyo(dir.path(), &["listening", "show", "words"], None);
    assert_eq!(
        (entries.code, entries.stderr.as_str()),
        (2, "inbox 里的 words 不是剧本\n")
    );
}
