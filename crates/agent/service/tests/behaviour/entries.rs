//! Entries leave the inbox when taken.

use agent::AgentError;
use agent_tools::InboxError;

use crate::support::{TestData, new_episode};

#[test]
fn taking_entries_gives_their_text_and_removes_them() {
    let data = TestData::new();
    let text = r#"{"writing": "諦める"}"#;
    data.put_entries("todo", text);

    assert_eq!(data.agent.take_entries("todo").unwrap(), text);
    assert!(data.agent.list().unwrap().is_empty());
}

#[test]
fn a_script_is_not_taken_as_entries() {
    let data = TestData::new();
    data.put_script("ep", new_episode(), "{}");

    let error = data.agent.take_entries("ep").unwrap_err();
    assert!(matches!(error, AgentError::NotEntries(ref name) if name == "ep"));
    assert_eq!(data.agent.list().unwrap().len(), 1);
}

#[test]
fn entries_already_taken_are_missing() {
    let data = TestData::new();
    let error = data.agent.take_entries("todo").unwrap_err();
    assert!(matches!(
        error,
        AgentError::Inbox(InboxError::Missing(ref name)) if name == "todo"
    ));
}
