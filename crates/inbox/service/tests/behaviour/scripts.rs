//! A script stays until it is saved or discarded; edits are written back.

use inbox::{InboxError, InboxScript, InboxSource};

use crate::support::{TestData, entries, new_episode};

const AGENT: InboxSource = InboxSource::Agent;

#[test]
fn reading_a_script_gives_its_target_and_keeps_it() {
    let data = TestData::new();
    data.put_script("ep", new_episode(), "一");

    let expected = InboxScript {
        target: new_episode(),
        text: "一".into(),
    };
    assert_eq!(
        data.inbox.read_script(AGENT, "ep").unwrap(),
        Some(expected.clone())
    );
    assert_eq!(data.inbox.require_script(AGENT, "ep").unwrap(), expected);
    assert_eq!(data.inbox.read_script(AGENT, "gone").unwrap(), None);
}

#[test]
fn entries_are_not_read_as_a_script() {
    let data = TestData::new();
    data.put_entries("todo", "{}");
    let error = data.inbox.read_script(AGENT, "todo").unwrap_err();
    assert!(matches!(error, InboxError::NotScript(ref name) if name == "todo"));
}

#[test]
fn a_script_no_longer_waiting_is_missing_when_required() {
    let data = TestData::new();
    let error = data.inbox.require_script(AGENT, "ep").unwrap_err();
    assert!(matches!(error, InboxError::Missing(ref name) if name == "ep"));
}

#[test]
fn replacing_needs_the_script_and_changes_it_whole() {
    let data = TestData::new();
    let error = data
        .inbox
        .replace_script(AGENT, "ep", new_episode(), "一")
        .unwrap_err();
    assert!(matches!(error, InboxError::Missing(ref name) if name == "ep"));

    data.put_script("ep", new_episode(), "一");
    data.inbox
        .replace_script(AGENT, "ep", new_episode(), "二")
        .unwrap();
    assert_eq!(data.inbox.require_script(AGENT, "ep").unwrap().text, "二");
    assert_eq!(data.inbox.list().unwrap().len(), 1);

    data.inbox.put(AGENT, "todo", &entries("{}")).unwrap();
    let error = data
        .inbox
        .replace_script(AGENT, "todo", new_episode(), "一")
        .unwrap_err();
    assert!(matches!(error, InboxError::NotScript(ref name) if name == "todo"));
}

#[test]
fn discarding_removes_a_script_once() {
    let data = TestData::new();
    data.put_script("ep", new_episode(), "一");

    data.inbox.discard_script(AGENT, "ep").unwrap();
    assert!(data.inbox.list().unwrap().is_empty());
    data.inbox.discard_script(AGENT, "ep").unwrap();
}

#[test]
fn entries_are_not_discarded_as_a_script() {
    let data = TestData::new();
    data.put_entries("todo", "{}");
    let error = data.inbox.discard_script(AGENT, "todo").unwrap_err();
    assert!(matches!(error, InboxError::NotScript(ref name) if name == "todo"));
    assert_eq!(data.inbox.list().unwrap().len(), 1);
}

#[test]
fn a_saved_script_is_removed() {
    let data = TestData::new();
    data.put_script("ep", new_episode(), "一");

    data.inbox.remove_saved_script(AGENT, "ep").unwrap();
    assert_eq!(data.inbox.read_script(AGENT, "ep").unwrap(), None);
}
