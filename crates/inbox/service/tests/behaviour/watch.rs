//! Watching tells of deliveries arriving, changing and leaving.

use std::sync::mpsc::{Receiver, channel};
use std::time::Duration;

use inbox::InboxSource;

use crate::support::{TestData, new_episode, script};

/// Long enough for the file system to report a change.
const WAIT: Duration = Duration::from_secs(5);

/// Waits for one report, then lets the rest of the same burst through: one
/// write may be reported more than once.
fn expect_change(changes: &Receiver<()>) {
    changes.recv_timeout(WAIT).expect("no change was reported");
    while changes.recv_timeout(Duration::from_millis(300)).is_ok() {}
}

#[test]
fn watching_creates_the_folders() {
    let data = TestData::new();
    let _watcher = data.inbox.watch(|| {}).unwrap();
    assert!(data.folder(InboxSource::Agent).is_dir());
    assert!(data.folder(InboxSource::Ai).is_dir());
}

#[test]
fn a_delivery_arriving_being_replaced_and_leaving_is_reported() {
    let data = TestData::new();
    let (sender, changes) = channel();
    let _watcher = data
        .inbox
        .watch(move || {
            let _ = sender.send(());
        })
        .unwrap();

    for source in [InboxSource::Agent, InboxSource::Ai] {
        data.inbox
            .put(source, "ep", &script(new_episode(), "{}"))
            .unwrap();
        expect_change(&changes);
        data.inbox
            .replace_script(source, "ep", new_episode(), "{\"title\": \"\"}")
            .unwrap();
        expect_change(&changes);
        data.inbox.discard_script(source, "ep").unwrap();
        expect_change(&changes);
    }
}
