//! `noobenkyo listening …`

use ai_tools::{Delivery, Inbox, ScriptTarget};
use serde_json::Value;

use crate::support::{SAMPLE_SCRIPT, noobenkyo, sample_episode, seeded_listening};

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

    let run = noobenkyo(
        dir.path(),
        &["listening", "send", "-", "--world", "火星"],
        Some(SAMPLE_SCRIPT),
    );
    assert_eq!((run.code, run.stdout.as_str()), (1, "世界 火星 不存在\n"));
    assert_eq!(inbox.take().unwrap(), None);

    let run = noobenkyo(
        dir.path(),
        &["listening", "send", "-", "--world", "星见町"],
        Some(SAMPLE_SCRIPT),
    );
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        inbox.take().unwrap(),
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
