//! Sending reaches the inbox only when everything checkable passes.

use ai_tools::{Delivery, Inbox, ScriptTarget, SendError, send_entries, send_script};

use crate::support::{SAMPLE_SCRIPT, listening, seeded};

fn new_episode(world: &str) -> ScriptTarget {
    ScriptTarget::NewEpisode {
        world: world.into(),
    }
}

fn problems(result: Result<(), SendError>) -> Vec<String> {
    match result {
        Err(SendError::Problems(problems)) => problems,
        other => panic!("expected problems, got {other:?}"),
    }
}

#[test]
fn entries_are_sent_even_with_warnings() {
    let dir = tempfile::tempdir().unwrap();
    let inbox = Inbox::in_data_dir(dir.path());
    let text = r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"], "sentences": [{"marked": "[[明日]]"}]}"#;
    assert!(!ai_tools::check_entries(text).is_empty());

    send_entries(&inbox, text).unwrap();
    assert_eq!(
        inbox.take().unwrap(),
        Some(Delivery::Entries { text: text.into() })
    );
}

#[tokio::test]
async fn a_passing_script_for_a_complete_world_is_sent() {
    let dir = tempfile::tempdir().unwrap();
    let listening = listening(&dir).await;
    seeded(&listening).await;
    let inbox = Inbox::in_data_dir(dir.path());

    send_script(&listening, &inbox, SAMPLE_SCRIPT, new_episode("星见町"))
        .await
        .unwrap();
    assert_eq!(
        inbox.take().unwrap(),
        Some(Delivery::Script {
            target: new_episode("星见町"),
            text: SAMPLE_SCRIPT.into(),
        })
    );
}

#[tokio::test]
async fn script_and_target_problems_come_together_and_nothing_is_sent() {
    let dir = tempfile::tempdir().unwrap();
    let listening = listening(&dir).await;
    seeded(&listening).await;
    let inbox = Inbox::in_data_dir(dir.path());

    let found = problems(send_script(&listening, &inbox, "{}", new_episode("火星")).await);
    assert_eq!(
        found,
        [
            "缺少 title",
            "缺少 summary",
            "缺少 lines",
            "世界 火星 不存在"
        ]
    );
    assert_eq!(inbox.take().unwrap(), None);
}

#[tokio::test]
async fn a_world_without_a_narrator_cannot_take_a_script() {
    let dir = tempfile::tempdir().unwrap();
    let listening = listening(&dir).await;
    seeded(&listening).await;
    let inbox = Inbox::in_data_dir(dir.path());

    let found = problems(send_script(&listening, &inbox, SAMPLE_SCRIPT, new_episode("雾都")).await);
    assert_eq!(found.len(), 1);
    assert!(found[0].starts_with("世界 雾都 缺少"), "{found:?}");
}

#[tokio::test]
async fn continuing_needs_the_episode() {
    let dir = tempfile::tempdir().unwrap();
    let listening = listening(&dir).await;
    seeded(&listening).await;
    let inbox = Inbox::in_data_dir(dir.path());
    let episode = listening
        .import_script("星见町", SAMPLE_SCRIPT)
        .await
        .unwrap();

    let append = ScriptTarget::Append { episode };
    send_script(&listening, &inbox, SAMPLE_SCRIPT, append.clone())
        .await
        .unwrap();
    assert_eq!(
        inbox.take().unwrap(),
        Some(Delivery::Script {
            target: append,
            text: SAMPLE_SCRIPT.into(),
        })
    );

    listening.delete_episode(episode).await.unwrap();
    let found = problems(
        send_script(
            &listening,
            &inbox,
            SAMPLE_SCRIPT,
            ScriptTarget::Append { episode },
        )
        .await,
    );
    assert_eq!(found, [format!("要续写的集 {episode} 不存在")]);
}
