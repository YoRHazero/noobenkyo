//! What every command shares: the two services, each opened in its own data
//! directory.

use std::path::Path;

use dictionary::{DictionaryError, DictionaryService};
use study::{StudyError, StudyService};
use tokio::sync::Mutex;

pub struct AppState {
    /// Not locked: its state lives in SQLite, which handles concurrent use.
    pub dictionary: DictionaryService,
    /// Held by a command from start to end, so no command sees study halfway
    /// through another one.
    pub study: Mutex<StudyService>,
}

/// Why the data could not be opened, in the features' own words.
#[derive(Debug, thiserror::Error)]
pub enum OpenError {
    #[error("{0}")]
    Dictionary(#[from] DictionaryError),
    #[error("{0}")]
    Study(#[from] StudyError),
}

impl AppState {
    pub async fn open(data_dir: &Path) -> Result<Self, OpenError> {
        Ok(Self {
            dictionary: DictionaryService::open(&data_dir.join("dictionary")).await?,
            study: Mutex::new(StudyService::open(&data_dir.join("study"))?),
        })
    }
}
