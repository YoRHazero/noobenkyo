//! What is waiting in the inbox.

use agent_tools::{Delivery, ScriptTarget};
use serde::Serialize;

use crate::{AgentError, AgentService};

/// One item waiting in the inbox, without its text: its kind tells the
/// frontend which preview opens it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum InboxItem {
    /// Entries, for the import dialog.
    Entries { name: String },
    /// A script, for the script preview, with where it goes.
    Script { name: String, target: ScriptTarget },
}

impl AgentService {
    /// Every item waiting, by when it was last written, oldest first. Leaves
    /// the files as they are. A file that cannot be understood fails the
    /// call; it is removed by then, so the next call goes on without it.
    pub fn list(&self) -> Result<Vec<InboxItem>, AgentError> {
        let mut items = Vec::new();
        for name in self.inbox.names()? {
            // Gone since the names were read: taken, saved or discarded.
            let Some(delivery) = self.inbox.read(&name)? else {
                continue;
            };
            items.push(match delivery {
                Delivery::Entries { .. } => InboxItem::Entries { name },
                Delivery::Script { target, .. } => InboxItem::Script { name, target },
            });
        }
        Ok(items)
    }
}
