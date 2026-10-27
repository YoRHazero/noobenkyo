//! Each source has its own folder: inbox-agent/ and inbox-ai/.

use std::time::Duration;

use inbox::{InboxError, InboxItem, InboxSource};

use crate::support::{TestData, entries, new_episode, script};

#[test]
fn each_source_keeps_its_deliveries_in_its_own_folder() {
    let data = TestData::new();
    data.inbox
        .put(InboxSource::Agent, "ep", &script(new_episode(), "agent"))
        .unwrap();
    data.inbox
        .put(InboxSource::Ai, "ep", &script(new_episode(), "ai"))
        .unwrap();

    assert!(data.folder(InboxSource::Agent).join("ep.json").is_file());
    assert!(data.folder(InboxSource::Ai).join("ep.json").is_file());
    for (source, text) in [(InboxSource::Agent, "agent"), (InboxSource::Ai, "ai")] {
        assert_eq!(data.inbox.require_script(source, "ep").unwrap().text, text);
    }
}

#[test]
fn the_list_has_both_sources_oldest_first() {
    let data = TestData::new();
    data.inbox
        .put(InboxSource::Ai, "ep", &script(new_episode(), "{}"))
        .unwrap();
    // Modification times on some file systems are coarse.
    std::thread::sleep(Duration::from_millis(20));
    data.inbox
        .put(InboxSource::Agent, "words", &entries("{}"))
        .unwrap();

    assert_eq!(
        data.inbox.list().unwrap(),
        [
            InboxItem::Script {
                source: InboxSource::Ai,
                name: "ep".into(),
                target: new_episode(),
            },
            InboxItem::Entries {
                source: InboxSource::Agent,
                name: "words".into(),
            },
        ]
    );
}

#[test]
fn a_name_is_looked_up_in_its_own_source_only() {
    let data = TestData::new();
    data.inbox
        .put(InboxSource::Ai, "words", &entries("{}"))
        .unwrap();

    let error = data
        .inbox
        .take_entries(InboxSource::Agent, "words")
        .unwrap_err();
    assert!(matches!(error, InboxError::Missing(_)));
    assert_eq!(
        data.inbox.take_entries(InboxSource::Ai, "words").unwrap(),
        "{}"
    );
}
