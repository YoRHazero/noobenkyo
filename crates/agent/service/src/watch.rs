//! Watching the inbox folder, so the app learns of new items while open.

use std::fs;

use agent_tools::{Inbox, InboxError};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::{AgentError, AgentService};

/// Watches the inbox for as long as it is kept; dropping it stops watching.
pub struct InboxWatcher {
    _watcher: RecommendedWatcher,
}

impl AgentService {
    /// Calls `on_change` whenever an item appears, is replaced or is
    /// removed. Files still being written are ignored; one write may still
    /// be reported more than once. Creates the folder first, since a folder
    /// that does not exist cannot be watched.
    pub fn watch(&self, on_change: impl Fn() + Send + 'static) -> Result<InboxWatcher, AgentError> {
        let dir = self.inbox.dir();
        fs::create_dir_all(dir).map_err(InboxError::from)?;
        let mut watcher = notify::recommended_watcher(move |result: notify::Result<Event>| {
            // An error from the watcher itself says nothing about the items.
            if let Ok(event) = result
                && touches_an_item(&event)
            {
                on_change();
            }
        })?;
        watcher.watch(dir, RecursiveMode::NonRecursive)?;
        Ok(InboxWatcher { _watcher: watcher })
    }
}

/// Whether the event changed an item's file; reading one does not count.
fn touches_an_item(event: &Event) -> bool {
    !matches!(event.kind, EventKind::Access(_))
        && event
            .paths
            .iter()
            .any(|path| Inbox::name_of(path).is_some())
}
