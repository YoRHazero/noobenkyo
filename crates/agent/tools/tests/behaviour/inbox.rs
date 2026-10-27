//! Deliveries in the inbox, by name.

use std::time::Duration;

use agent_tools::{Delivery, Inbox, InboxError, ScriptTarget};

fn entries(text: &str) -> Delivery {
    Delivery::Entries { text: text.into() }
}

fn script(text: &str) -> Delivery {
    Delivery::Script {
        target: ScriptTarget::NewEpisode {
            world: "星见町".into(),
        },
        text: text.into(),
    }
}

#[test]
fn a_delivery_is_kept_under_its_name_until_removed() {
    let dir = tempfile::tempdir().unwrap();
    let inbox = Inbox::in_data_dir(dir.path());
    inbox.put("ep", &script("一")).unwrap();

    assert_eq!(inbox.read("ep").unwrap(), Some(script("一")));
    assert_eq!(inbox.read("ep").unwrap(), Some(script("一")));
    assert!(inbox.remove("ep").unwrap());
    assert_eq!(inbox.read("ep").unwrap(), None);
    assert!(!inbox.remove("ep").unwrap());
}

#[test]
fn a_name_is_used_once_until_its_delivery_leaves() {
    let dir = tempfile::tempdir().unwrap();
    let inbox = Inbox::in_data_dir(dir.path());
    inbox.put("ep", &script("一")).unwrap();

    let error = inbox.put("ep", &entries("二")).unwrap_err();
    assert!(matches!(error, InboxError::Taken(ref name) if name == "ep"));
    assert_eq!(inbox.read("ep").unwrap(), Some(script("一")));

    inbox.remove("ep").unwrap();
    inbox.put("ep", &entries("二")).unwrap();
}

#[test]
fn replacing_needs_the_delivery_and_changes_it_whole() {
    let dir = tempfile::tempdir().unwrap();
    let inbox = Inbox::in_data_dir(dir.path());
    let error = inbox.replace("ep", &script("一")).unwrap_err();
    assert!(matches!(error, InboxError::Missing(ref name) if name == "ep"));

    inbox.put("ep", &script("一")).unwrap();
    inbox.replace("ep", &script("二")).unwrap();
    assert_eq!(inbox.read("ep").unwrap(), Some(script("二")));
    assert_eq!(inbox.names().unwrap(), ["ep"]);
}

#[test]
fn names_come_oldest_first_by_last_write() {
    let dir = tempfile::tempdir().unwrap();
    let inbox = Inbox::in_data_dir(dir.path());
    // Modification times on some file systems are coarse.
    let pause = || std::thread::sleep(Duration::from_millis(20));
    inbox.put("b", &script("一")).unwrap();
    pause();
    inbox.put("a", &entries("二")).unwrap();
    assert_eq!(inbox.names().unwrap(), ["b", "a"]);

    pause();
    inbox.replace("b", &script("三")).unwrap();
    assert_eq!(inbox.names().unwrap(), ["a", "b"]);
}

#[test]
fn an_inbox_never_written_is_empty() {
    let dir = tempfile::tempdir().unwrap();
    let inbox = Inbox::in_data_dir(dir.path());
    assert!(inbox.names().unwrap().is_empty());
    assert_eq!(inbox.read("ep").unwrap(), None);
}

#[test]
fn files_still_being_written_are_left_alone() {
    let dir = tempfile::tempdir().unwrap();
    let inbox = Inbox::in_data_dir(dir.path());
    std::fs::create_dir_all(inbox.dir()).unwrap();
    std::fs::write(inbox.dir().join(".ep.1234.tmp"), "{").unwrap();
    assert!(inbox.names().unwrap().is_empty());
}

#[test]
fn names_that_would_hide_or_escape_the_file_are_refused() {
    let dir = tempfile::tempdir().unwrap();
    let inbox = Inbox::in_data_dir(dir.path());
    for name in ["", ".ep", "../ep", "a/b", "a\\b"] {
        let error = inbox.put(name, &script("一")).unwrap_err();
        assert!(matches!(error, InboxError::BadName(_)), "{name:?}");
    }
}

#[test]
fn a_file_that_cannot_be_read_is_removed_and_reported() {
    let dir = tempfile::tempdir().unwrap();
    let inbox = Inbox::in_data_dir(dir.path());
    inbox.put("ep", &entries("二")).unwrap();
    std::fs::write(inbox.dir().join("bad.json"), "not json").unwrap();

    let error = inbox.read("bad").unwrap_err();
    assert!(matches!(error, InboxError::Unreadable { ref file, .. } if file == "bad.json"));
    assert_eq!(inbox.names().unwrap(), ["ep"]);
}
