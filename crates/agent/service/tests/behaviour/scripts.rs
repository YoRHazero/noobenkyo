//! A script stays until it is saved or discarded; edits are written back.

use agent::{AgentError, InboxScript, ScriptTarget};
use agent_tools::InboxError;

use crate::support::{SAMPLE_SCRIPT, TestData, WORLD, new_episode};

/// The sample script with another title, as an edit in the preview makes it.
fn retitled(title: &str) -> String {
    SAMPLE_SCRIPT.replace("海边的早晨", title)
}

#[test]
fn reading_a_script_gives_its_target_and_keeps_it() {
    let data = TestData::new();
    data.put_script("ep", new_episode(), SAMPLE_SCRIPT);

    let expected = InboxScript {
        target: new_episode(),
        text: SAMPLE_SCRIPT.into(),
    };
    assert_eq!(
        data.agent.read_script("ep").unwrap(),
        Some(expected.clone())
    );
    assert_eq!(data.agent.read_script("ep").unwrap(), Some(expected));
    assert_eq!(data.agent.read_script("gone").unwrap(), None);
}

#[test]
fn entries_are_not_read_as_a_script() {
    let data = TestData::new();
    data.put_entries("todo", "{}");
    let error = data.agent.read_script("todo").unwrap_err();
    assert!(matches!(error, AgentError::NotScript(ref name) if name == "todo"));
}

#[tokio::test]
async fn a_passing_edit_is_written_back() {
    let data = TestData::new();
    let listening = data.listening().await;
    data.put_script("ep", new_episode(), SAMPLE_SCRIPT);

    let edited = retitled("灯塔之约");
    let problems = data
        .agent
        .replace_script(&listening, "ep", new_episode(), &edited)
        .await
        .unwrap();
    assert!(problems.is_empty(), "{problems:?}");
    assert_eq!(data.agent.read_script("ep").unwrap().unwrap().text, edited);
}

#[tokio::test]
async fn a_failing_edit_returns_its_problems_and_is_not_written() {
    let data = TestData::new();
    let listening = data.listening().await;
    data.put_script("ep", new_episode(), SAMPLE_SCRIPT);

    let problems = data
        .agent
        .replace_script(&listening, "ep", new_episode(), &retitled(""))
        .await
        .unwrap();
    assert!(!problems.is_empty());
    assert_eq!(
        data.agent.read_script("ep").unwrap().unwrap().text,
        SAMPLE_SCRIPT
    );
}

#[tokio::test]
async fn saving_stores_a_new_episode_from_the_text_given_and_removes_the_file() {
    let data = TestData::new();
    let listening = data.listening().await;
    data.put_script("ep", new_episode(), SAMPLE_SCRIPT);

    let episode = data
        .agent
        .save_script(&listening, "ep", new_episode(), &retitled("灯塔之约"))
        .await
        .unwrap();
    let stored = listening.episode(episode).await.unwrap().unwrap();
    assert_eq!(stored.world, WORLD);
    assert_eq!(stored.title, "灯塔之约");
    assert_eq!(stored.lines.len(), 8);
    assert!(data.agent.list().unwrap().is_empty());
}

#[tokio::test]
async fn saving_a_continuation_appends_and_returns_the_episode() {
    let data = TestData::new();
    let listening = data.listening().await;
    let episode = listening.import_script(WORLD, SAMPLE_SCRIPT).await.unwrap();
    let target = ScriptTarget::Append { episode };
    data.put_script("more", target.clone(), SAMPLE_SCRIPT);

    let saved = data
        .agent
        .save_script(&listening, "more", target, SAMPLE_SCRIPT)
        .await
        .unwrap();
    assert_eq!(saved, episode);
    let stored = listening.episode(episode).await.unwrap().unwrap();
    assert_eq!(stored.lines.len(), 16);
    assert!(data.agent.list().unwrap().is_empty());
}

#[tokio::test]
async fn a_script_no_longer_waiting_is_not_stored_again() {
    let data = TestData::new();
    let listening = data.listening().await;

    let error = data
        .agent
        .save_script(&listening, "ep", new_episode(), SAMPLE_SCRIPT)
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        AgentError::Inbox(InboxError::Missing(ref name)) if name == "ep"
    ));
    assert!(listening.episodes(WORLD).await.unwrap().is_empty());
}

#[tokio::test]
async fn a_script_that_cannot_be_stored_stays_in_the_inbox() {
    let data = TestData::new();
    let listening = data.listening().await;
    data.put_script("ep", new_episode(), SAMPLE_SCRIPT);

    let error = data
        .agent
        .save_script(&listening, "ep", new_episode(), &retitled(""))
        .await
        .unwrap_err();
    assert!(matches!(error, AgentError::Listening(_)), "{error:?}");
    assert!(data.agent.read_script("ep").unwrap().is_some());
}

#[test]
fn discarding_removes_a_script_once() {
    let data = TestData::new();
    data.put_script("ep", new_episode(), SAMPLE_SCRIPT);

    data.agent.discard_script("ep").unwrap();
    assert!(data.agent.list().unwrap().is_empty());
    data.agent.discard_script("ep").unwrap();
}

#[test]
fn entries_are_not_discarded_as_a_script() {
    let data = TestData::new();
    data.put_entries("todo", "{}");
    let error = data.agent.discard_script("todo").unwrap_err();
    assert!(matches!(error, AgentError::NotScript(ref name) if name == "todo"));
    assert_eq!(data.agent.list().unwrap().len(), 1);
}
