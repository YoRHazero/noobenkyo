//! Deliveries in the inbox, by name.

use std::time::Duration;

use inbox::{InboxError, InboxItem, InboxScript, InboxSource};

use crate::support::{TestData, entries, new_episode, script};

const AGENT: InboxSource = InboxSource::Agent;

fn waiting(text: &str) -> InboxScript {
    InboxScript {
        target: new_episode(),
        text: text.into(),
    }
}

/// Names in the list, in its order.
fn names(data: &TestData) -> Vec<String> {
    data.inbox
        .list()
        .unwrap()
        .into_iter()
        .map(|item| match item {
            InboxItem::Entries { name, .. } | InboxItem::Script { name, .. } => name,
        })
        .collect()
}

#[test]
fn a_delivery_is_kept_under_its_name_until_removed() {
    let data = TestData::new();
    data.put_script("ep", new_episode(), "一");

    assert_eq!(
        data.inbox.read_script(AGENT, "ep").unwrap(),
        Some(waiting("一"))
    );
    assert_eq!(
        data.inbox.read_script(AGENT, "ep").unwrap(),
        Some(waiting("一"))
    );
    data.inbox.discard_script(AGENT, "ep").unwrap();
    assert_eq!(data.inbox.read_script(AGENT, "ep").unwrap(), None);
}

#[test]
fn a_name_is_used_once_until_its_delivery_leaves() {
    let data = TestData::new();
    data.put_script("ep", new_episode(), "一");

    let error = data.inbox.put(AGENT, "ep", &entries("二")).unwrap_err();
    assert!(matches!(error, InboxError::Taken(ref name) if name == "ep"));
    assert_eq!(
        data.inbox.read_script(AGENT, "ep").unwrap(),
        Some(waiting("一"))
    );

    data.inbox.discard_script(AGENT, "ep").unwrap();
    data.inbox.put(AGENT, "ep", &entries("二")).unwrap();
}

#[test]
fn names_come_oldest_first_by_last_write() {
    let data = TestData::new();
    // Modification times on some file systems are coarse.
    let pause = || std::thread::sleep(Duration::from_millis(20));
    data.put_script("b", new_episode(), "一");
    pause();
    data.put_entries("a", "二");
    assert_eq!(names(&data), ["b", "a"]);

    pause();
    data.inbox
        .replace_script(AGENT, "b", new_episode(), "三")
        .unwrap();
    assert_eq!(names(&data), ["a", "b"]);
}

#[test]
fn an_inbox_never_written_is_empty() {
    let data = TestData::new();
    assert!(data.inbox.list().unwrap().is_empty());
    assert_eq!(data.inbox.read_script(AGENT, "ep").unwrap(), None);
}

#[test]
fn files_still_being_written_are_left_alone() {
    let data = TestData::new();
    let folder = data.folder(AGENT);
    std::fs::create_dir_all(&folder).unwrap();
    std::fs::write(folder.join(".ep.1234.tmp"), "{").unwrap();
    assert!(data.inbox.list().unwrap().is_empty());
}

#[test]
fn names_that_would_hide_or_escape_the_file_are_refused() {
    let data = TestData::new();
    for name in ["", ".ep", "../ep", "a/b", "a\\b"] {
        let error = data
            .inbox
            .put(AGENT, name, &script(new_episode(), "一"))
            .unwrap_err();
        assert!(matches!(error, InboxError::BadName(_)), "{name:?}");
    }
}

#[test]
fn a_file_that_cannot_be_read_is_removed_and_reported() {
    let data = TestData::new();
    data.put_entries("ep", "二");
    std::fs::write(data.folder(AGENT).join("bad.json"), "not json").unwrap();

    let error = data.inbox.read_script(AGENT, "bad").unwrap_err();
    assert!(matches!(error, InboxError::Unreadable { ref file, .. } if file == "bad.json"));
    assert_eq!(names(&data), ["ep"]);
}
