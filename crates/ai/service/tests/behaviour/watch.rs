//! Watching tells of deliveries arriving, changing and leaving.

use std::sync::mpsc::{Receiver, channel};
use std::time::Duration;

use crate::support::{SAMPLE_SCRIPT, TestData, new_episode};

/// Long enough for the file system to report a change.
const WAIT: Duration = Duration::from_secs(5);

/// Waits for one report, then lets the rest of the same burst through: one
/// write may be reported more than once.
fn expect_change(changes: &Receiver<()>) {
    changes.recv_timeout(WAIT).expect("no change was reported");
    while changes.recv_timeout(Duration::from_millis(300)).is_ok() {}
}

#[test]
fn watching_creates_the_folder() {
    let data = TestData::new();
    let _watcher = data.ai.watch(|| {}).unwrap();
    assert!(data.inbox.dir().is_dir());
}

#[test]
fn a_delivery_arriving_being_replaced_and_leaving_is_reported() {
    let data = TestData::new();
    let (sender, changes) = channel();
    let _watcher = data
        .ai
        .watch(move || {
            let _ = sender.send(());
        })
        .unwrap();

    data.put_script("ep", new_episode(), SAMPLE_SCRIPT);
    expect_change(&changes);
    data.inbox
        .replace(
            "ep",
            &ai_tools::Delivery::Script {
                target: new_episode(),
                text: "{}".into(),
            },
        )
        .unwrap();
    expect_change(&changes);
    data.ai.discard_script("ep").unwrap();
    expect_change(&changes);
}
