//! What is waiting in the inbox.

use serde::Serialize;

use crate::{Delivery, InboxError, InboxService, InboxSource, ScriptTarget};

/// One item waiting in the inbox, without its text: its kind tells the
/// frontend which preview opens it, its source and name which file it is.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum InboxItem {
    /// Entries, for the import dialog.
    Entries { source: InboxSource, name: String },
    /// A script, for the script preview, with where it goes.
    Script {
        source: InboxSource,
        name: String,
        target: ScriptTarget,
    },
}

impl InboxService {
    /// Every item waiting from every source, by when it was last written,
    /// oldest first. Leaves the files as they are. A file that cannot be
    /// understood fails the call; it is removed by then, so the next call
    /// goes on without it.
    pub fn list(&self) -> Result<Vec<InboxItem>, InboxError> {
        let mut waiting = Vec::new();
        for source in InboxSource::ALL {
            for (written, name) in self.folder(source).waiting()? {
                waiting.push((written, name, source));
            }
        }
        waiting.sort();
        let mut items = Vec::new();
        for (_, name, source) in waiting {
            // Gone since the folder was read: taken, saved or discarded.
            let Some(delivery) = self.folder(source).read(&name)? else {
                continue;
            };
            items.push(match delivery {
                Delivery::Entries { .. } => InboxItem::Entries { source, name },
                Delivery::Script { target, .. } => InboxItem::Script {
                    source,
                    name,
                    target,
                },
            });
        }
        Ok(items)
    }
}
