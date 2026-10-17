use std::path::Path;

use dictionary::DictionaryService;
use study::StudyService;
use tokio::sync::Mutex;

use crate::command_error::CommandError;

/// Each feature gets its own data directory.
pub struct AppState {
    pub dictionary: DictionaryService,
    /// Locked for a whole flow, so study operations never interleave.
    pub study: Mutex<StudyService>,
}

impl AppState {
    pub async fn open(data_dir: &Path) -> Result<Self, CommandError> {
        Ok(Self {
            dictionary: DictionaryService::open(&data_dir.join("dictionary")).await?,
            study: Mutex::new(StudyService::open(&data_dir.join("study"))?),
        })
    }
}
