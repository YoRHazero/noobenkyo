//! Watching the inbox's folders, so the app learns of new items while open.

use std::fs;

use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::folder::Folder;
use crate::{InboxError, InboxService, InboxSource};

/// Watches the inbox for as long as it is kept; dropping it stops watching.
pub struct InboxWatcher {
    _watcher: RecommendedWatcher,
}

impl InboxService {
    /// Calls `on_change` whenever an item from any source appears, is
    /// replaced or is removed. Files still being written are ignored; one
    /// write may still be reported more than once. Creates the folders
    /// first, since a folder that does not exist cannot be watched.
    pub fn watch(&self, on_change: impl Fn() + Send + 'static) -> Result<InboxWatcher, InboxError> {
        for source in InboxSource::ALL {
            fs::create_dir_all(self.folder(source).dir())?;
        }
        let mut watcher = notify::recommended_watcher(move |result: notify::Result<Event>| {
            // An error from the watcher itself says nothing about the items.
            if let Ok(event) = result
                && touches_an_item(&event)
            {
                on_change();
            }
        })?;
        for source in InboxSource::ALL {
            watcher.watch(self.folder(source).dir(), RecursiveMode::NonRecursive)?;
        }
        Ok(InboxWatcher { _watcher: watcher })
    }
}

/// Whether the event changed an item's file; reading one does not count.
fn touches_an_item(event: &Event) -> bool {
    !matches!(event.kind, EventKind::Access(_))
        && event
            .paths
            .iter()
            .any(|path| Folder::name_of(path).is_some())
}
