//! What is waiting, without the texts.

use std::time::Duration;

use inbox::{InboxError, InboxItem, InboxSource};

use crate::support::{TestData, new_episode};

#[test]
fn an_inbox_never_written_has_nothing_waiting() {
    let data = TestData::new();
    assert!(data.inbox.list().unwrap().is_empty());
}

#[test]
fn items_come_oldest_first_with_their_kind_and_target() {
    let data = TestData::new();
    data.put_script("ep", new_episode(), "{}");
    // Modification times on some file systems are coarse.
    std::thread::sleep(Duration::from_millis(20));
    data.put_entries("todo", "{}");

    assert_eq!(
        data.inbox.list().unwrap(),
        [
            InboxItem::Script {
                source: InboxSource::Agent,
                name: "ep".into(),
                target: new_episode(),
            },
            InboxItem::Entries {
                source: InboxSource::Agent,
                name: "todo".into()
            },
        ]
    );
}

#[test]
fn a_file_that_cannot_be_read_fails_the_list_once() {
    let data = TestData::new();
    data.put_entries("todo", "{}");
    std::fs::write(data.folder(InboxSource::Agent).join("bad.json"), "not json").unwrap();

    let error = data.inbox.list().unwrap_err();
    assert!(matches!(
        error,
        InboxError::Unreadable { ref file, .. } if file == "bad.json"
    ));
    assert_eq!(
        data.inbox.list().unwrap(),
        [InboxItem::Entries {
            source: InboxSource::Agent,
            name: "todo".into()
        }]
    );
}
