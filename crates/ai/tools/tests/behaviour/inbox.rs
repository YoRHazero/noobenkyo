//! Putting deliveries in the inbox and taking them out.

use ai_tools::{Delivery, Inbox, InboxError, ScriptTarget};

fn entries(text: &str) -> Delivery {
    Delivery::Entries { text: text.into() }
}

#[test]
fn deliveries_come_out_oldest_first_and_once() {
    let dir = tempfile::tempdir().unwrap();
    let inbox = Inbox::in_data_dir(dir.path());
    let script = Delivery::Script {
        target: ScriptTarget::NewEpisode {
            world: "星见町".into(),
        },
        text: "{}".into(),
    };
    inbox.put(&entries("一")).unwrap();
    // Names carry the millisecond they were put in.
    std::thread::sleep(std::time::Duration::from_millis(2));
    inbox.put(&script).unwrap();

    assert_eq!(inbox.take().unwrap(), Some(entries("一")));
    assert_eq!(inbox.take().unwrap(), Some(script));
    assert_eq!(inbox.take().unwrap(), None);
}

#[test]
fn an_inbox_never_written_is_empty() {
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(Inbox::in_data_dir(dir.path()).take().unwrap(), None);
}

#[test]
fn files_still_being_written_are_left_alone() {
    let dir = tempfile::tempdir().unwrap();
    let inbox = Inbox::in_data_dir(dir.path());
    std::fs::create_dir_all(inbox.dir()).unwrap();
    std::fs::write(inbox.dir().join(".00000000000000000001-x.tmp"), "{").unwrap();
    assert_eq!(inbox.take().unwrap(), None);
}

#[test]
fn a_file_that_cannot_be_read_is_removed_and_reported() {
    let dir = tempfile::tempdir().unwrap();
    let inbox = Inbox::in_data_dir(dir.path());
    inbox.put(&entries("二")).unwrap();
    std::fs::write(inbox.dir().join("0.json"), "not json").unwrap();

    let error = inbox.take().unwrap_err();
    assert!(matches!(error, InboxError::Unreadable { ref file, .. } if file == "0.json"));
    assert_eq!(inbox.take().unwrap(), Some(entries("二")));
}
