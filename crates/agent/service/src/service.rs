use std::path::Path;

use agent_tools::Inbox;

/// The app's side of the inbox: listing what is waiting, reading scripts
/// and taking entries out, writing back, saving and discarding scripts, and
/// watching for changes. Reading leaves a file as it is; taking removes it.
/// It opens no data; listening's service is handed in by the methods that
/// need it.
pub struct AgentService {
    pub(crate) inbox: Inbox,
}

impl AgentService {
    /// The service over the inbox in the app's data directory, where the
    /// command line leaves what agents send. Nothing is read or created yet.
    pub fn new(data_dir: &Path) -> Self {
        Self {
            inbox: Inbox::in_data_dir(data_dir),
        }
    }
}
