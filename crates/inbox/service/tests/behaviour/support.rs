//! An inbox in a temporary data directory, and deliveries to leave in it.

use std::path::PathBuf;

use inbox::{Delivery, InboxService, InboxSource, ScriptTarget};

/// The world new episodes go to.
pub const WORLD: &str = "星见町";

/// The inbox over a data directory of its own.
pub struct TestData {
    pub dir: tempfile::TempDir,
    pub inbox: InboxService,
}

impl TestData {
    pub fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        Self {
            inbox: InboxService::new(dir.path()),
            dir,
        }
    }

    /// Where the app keeps what `source` delivers.
    pub fn folder(&self, source: InboxSource) -> PathBuf {
        self.dir.path().join(match source {
            InboxSource::Agent => "inbox-agent",
            InboxSource::Ai => "inbox-ai",
        })
    }

    pub fn put_entries(&self, name: &str, text: &str) {
        self.inbox
            .put(InboxSource::Agent, name, &entries(text))
            .unwrap();
    }

    pub fn put_script(&self, name: &str, target: ScriptTarget, text: &str) {
        self.inbox
            .put(InboxSource::Agent, name, &script(target, text))
            .unwrap();
    }
}

pub fn entries(text: &str) -> Delivery {
    Delivery::Entries { text: text.into() }
}

pub fn script(target: ScriptTarget, text: &str) -> Delivery {
    Delivery::Script {
        target,
        text: text.into(),
    }
}

pub fn new_episode() -> ScriptTarget {
    ScriptTarget::NewEpisode {
        world: WORLD.into(),
    }
}
