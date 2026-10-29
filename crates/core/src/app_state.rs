//! What every command shares: the services, each over its own place in the
//! data directory.

use std::path::Path;

use ai::{AiError, AiService};
use dictionary::{DictionaryError, DictionaryService};
use inbox::InboxService;
use listening::{ListeningError, ListeningService};
use study::{StudyError, StudyService};
use tokio::sync::Mutex;

use crate::tasks::Tasks;

pub struct AppState {
    /// Not locked: its state lives in SQLite, which handles concurrent use.
    pub dictionary: DictionaryService,
    /// Not locked, for the same reason.
    pub listening: ListeningService,
    /// Held by a command from start to end, so no command sees study halfway
    /// through another one.
    pub study: Mutex<StudyService>,
    /// Not locked: every inbox file is written whole and renamed into place.
    pub inbox: InboxService,
    /// Not locked: it guards its own invoker.
    pub ai: AiService,
    /// The long jobs running in the background, core's own.
    pub tasks: Tasks,
}

/// Why the data could not be opened, in the features' own words.
#[derive(Debug, thiserror::Error)]
pub enum OpenError {
    #[error("{0}")]
    Dictionary(#[from] DictionaryError),
    #[error("{0}")]
    Listening(#[from] ListeningError),
    #[error("{0}")]
    Study(#[from] StudyError),
    #[error("{0}")]
    Ai(#[from] AiError),
}

impl AppState {
    pub async fn open(data_dir: &Path) -> Result<Self, OpenError> {
        Ok(Self {
            dictionary: DictionaryService::open(&data_dir.join("dictionary")).await?,
            listening: ListeningService::open(&data_dir.join("listening")).await?,
            study: Mutex::new(StudyService::open(&data_dir.join("study"))?),
            inbox: InboxService::new(data_dir),
            ai: AiService::open(&data_dir.join("ai"))?,
            tasks: Tasks::default(),
        })
    }
}
