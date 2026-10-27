use serde::{Deserialize, Serialize};

/// Who delivered an item. Each source has its own folder, so the same name
/// may wait from both.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, specta::Type,
)]
#[serde(rename_all = "camelCase")]
pub enum InboxSource {
    /// An outside agent, through the command line.
    Agent,
    /// The app's own models.
    Ai,
}

impl InboxSource {
    pub(crate) const ALL: [InboxSource; 2] = [InboxSource::Agent, InboxSource::Ai];

    /// The source's folder name inside the app's data directory.
    pub(crate) fn dir_name(self) -> &'static str {
        match self {
            InboxSource::Agent => "inbox-agent",
            InboxSource::Ai => "inbox-ai",
        }
    }
}
