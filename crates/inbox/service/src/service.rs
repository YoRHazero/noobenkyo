use std::path::Path;

use crate::InboxSource;
use crate::folder::Folder;

/// The inbox: leaving deliveries, listing what is waiting, reading scripts
/// and taking entries out, writing scripts back, removing them, and watching
/// for changes. Reading leaves a file as it is; taking removes it. It opens
/// no other data: whoever checks or stores a delivery does that first.
pub struct InboxService {
    agent: Folder,
    ai: Folder,
}

impl InboxService {
    /// The inbox in the app's data directory: inbox-agent/ for what agents
    /// send through the command line, inbox-ai/ for what the app's own
    /// models write. Nothing is read or created yet.
    pub fn new(data_dir: &Path) -> Self {
        let folder = |source: InboxSource| Folder::new(data_dir.join(source.dir_name()));
        Self {
            agent: folder(InboxSource::Agent),
            ai: folder(InboxSource::Ai),
        }
    }

    pub(crate) fn folder(&self, source: InboxSource) -> &Folder {
        match source {
            InboxSource::Agent => &self.agent,
            InboxSource::Ai => &self.ai,
        }
    }
}
