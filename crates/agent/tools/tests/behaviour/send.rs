//! Sending reaches the inbox only when everything checkable passes.

use agent_tools::{SendError, replace_script, send_entries, send_script};
use inbox::{InboxError, InboxScript, InboxService, InboxSource, ScriptTarget};

use crate::support::{SAMPLE_SCRIPT, listening, seeded};

const AGENT: InboxSource = InboxSource::Agent;

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
    let inbox = InboxService::new(dir.path());
    let text = r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"], "sentences": [{"marked": "[[明日]]"}]}"#;
    assert!(!agent_tools::check_entries(text).is_empty());

    send_entries(&inbox, "words", text).unwrap();
    assert_eq!(inbox.take_entries(AGENT, "words").unwrap(), text);
}

#[tokio::test]
async fn a_passing_script_for_a_complete_world_is_sent() {
    let dir = tempfile::tempdir().unwrap();
    let listening = listening(&dir).await;
    seeded(&listening).await;
    let inbox = InboxService::new(dir.path());

    send_script(
        &listening,
        &inbox,
        "ep",
        SAMPLE_SCRIPT,
        new_episode("星见町"),
    )
    .await
    .unwrap();
    assert_eq!(
        inbox.read_script(AGENT, "ep").unwrap(),
        Some(InboxScript {
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
    let inbox = InboxService::new(dir.path());

    let found = problems(send_script(&listening, &inbox, "ep", "{}", new_episode("火星")).await);
    assert_eq!(
        found,
        [
            "缺少 title",
            "缺少 summary",
            "缺少 lines",
            "世界 火星 不存在"
        ]
    );
    assert!(inbox.list().unwrap().is_empty());
}

#[tokio::test]
async fn a_world_without_a_narrator_cannot_take_a_script() {
    let dir = tempfile::tempdir().unwrap();
    let listening = listening(&dir).await;
    seeded(&listening).await;
    let inbox = InboxService::new(dir.path());

    let found =
        problems(send_script(&listening, &inbox, "ep", SAMPLE_SCRIPT, new_episode("雾都")).await);
    assert_eq!(found.len(), 1);
    assert!(found[0].starts_with("世界 雾都 缺少"), "{found:?}");
}

#[tokio::test]
async fn continuing_needs_the_episode() {
    let dir = tempfile::tempdir().unwrap();
    let listening = listening(&dir).await;
    seeded(&listening).await;
    let inbox = InboxService::new(dir.path());
    let episode = listening
        .import_script("星见町", SAMPLE_SCRIPT)
        .await
        .unwrap();

    let append = ScriptTarget::Append { episode };
    send_script(&listening, &inbox, "ep", SAMPLE_SCRIPT, append.clone())
        .await
        .unwrap();
    assert_eq!(
        inbox.read_script(AGENT, "ep").unwrap(),
        Some(InboxScript {
            target: append,
            text: SAMPLE_SCRIPT.into(),
        })
    );

    listening.delete_episode(episode).await.unwrap();
    let found = problems(
        send_script(
            &listening,
            &inbox,
            "ep2",
            SAMPLE_SCRIPT,
            ScriptTarget::Append { episode },
        )
        .await,
    );
    assert_eq!(found, [format!("要续写的集 {episode} 不存在")]);
}

#[tokio::test]
async fn a_waiting_script_is_replaced_after_the_same_checks() {
    let dir = tempfile::tempdir().unwrap();
    let listening = listening(&dir).await;
    seeded(&listening).await;
    let inbox = InboxService::new(dir.path());
    send_script(
        &listening,
        &inbox,
        "ep",
        SAMPLE_SCRIPT,
        new_episode("星见町"),
    )
    .await
    .unwrap();

    let found =
        problems(replace_script(&listening, &inbox, "ep", "{}", new_episode("星见町")).await);
    assert_eq!(found, ["缺少 title", "缺少 summary", "缺少 lines"]);
    let edited = SAMPLE_SCRIPT.replace("海边的早晨", "下雨的早晨");
    replace_script(&listening, &inbox, "ep", &edited, new_episode("星见町"))
        .await
        .unwrap();
    assert_eq!(
        inbox.read_script(AGENT, "ep").unwrap(),
        Some(InboxScript {
            target: new_episode("星见町"),
            text: edited,
        })
    );
}

#[tokio::test]
async fn only_a_waiting_script_can_be_replaced() {
    let dir = tempfile::tempdir().unwrap();
    let listening = listening(&dir).await;
    seeded(&listening).await;
    let inbox = InboxService::new(dir.path());

    // Said before any check, so a failing text does not hide it.
    let missing = replace_script(&listening, &inbox, "ep", "{}", new_episode("星见町")).await;
    assert!(
        matches!(missing, Err(SendError::Inbox(InboxError::Missing(_)))),
        "{missing:?}"
    );

    send_entries(&inbox, "ep", "{\"writing\": \"明日\"}").unwrap();
    let entries = replace_script(
        &listening,
        &inbox,
        "ep",
        SAMPLE_SCRIPT,
        new_episode("星见町"),
    )
    .await;
    assert!(
        matches!(entries, Err(SendError::Inbox(InboxError::NotScript(_)))),
        "{entries:?}"
    );
}

#[tokio::test]
async fn sending_under_a_waiting_name_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let listening = listening(&dir).await;
    seeded(&listening).await;
    let inbox = InboxService::new(dir.path());
    send_entries(&inbox, "ep", "{\"writing\": \"明日\"}").unwrap();

    let taken = send_script(
        &listening,
        &inbox,
        "ep",
        SAMPLE_SCRIPT,
        new_episode("星见町"),
    )
    .await;
    assert!(
        matches!(taken, Err(SendError::Inbox(InboxError::Taken(_)))),
        "{taken:?}"
    );
}
