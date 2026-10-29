use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, PoisonError};

use ai_invoker::Invoker;

use crate::{AiError, AiSettings, MachineProbe, settings_file};

/// The app's own use of models: its settings and the invoker built from
/// them. Touches only its own data directory.
pub struct AiService {
    dir: PathBuf,
    loaded: Mutex<Loaded>,
    /// Every call holds the read lock from start to end, waiting for a
    /// machine included, so new settings never land under a running call.
    pub(crate) invoker: tokio::sync::RwLock<Invoker>,
    /// What the last probe of all the machines found.
    pub(crate) reports: Mutex<Vec<MachineProbe>>,
}

struct Loaded {
    settings: AiSettings,
    /// Why the file found at startup was not used, until settings replace it.
    problem: Option<String>,
}

impl AiService {
    /// Reads ai/settings.json. Without it there are no machines. A file that
    /// cannot be used does not stop the app: there are no machines either,
    /// and every call says what is wrong with the file.
    pub fn open(dir: &Path) -> Result<AiService, AiError> {
        std::fs::create_dir_all(dir)?;
        let (settings, problem) = match settings_file::load(dir) {
            Ok(settings) => (settings, None),
            Err(problem) => (AiSettings::default(), Some(problem)),
        };
        Ok(AiService {
            dir: dir.to_owned(),
            invoker: tokio::sync::RwLock::new(Invoker::new(settings.invoker.clone())),
            loaded: Mutex::new(Loaded { settings, problem }),
            reports: Mutex::new(Vec::new()),
        })
    }

    pub fn settings(&self) -> AiSettings {
        self.loaded().settings.clone()
    }

    /// Replaces the settings as a whole; invalid ones are rejected and the
    /// old ones kept. Refused while any call is under way or waiting, since
    /// the executors it uses close with the old invoker.
    pub fn update_settings(&self, new: AiSettings) -> Result<(), AiError> {
        new.validate()?;
        let mut invoker = self.invoker.try_write().map_err(|_| AiError::Busy)?;
        settings_file::save(&self.dir, &new)?;
        *invoker = Invoker::new(new.invoker.clone());
        *self.loaded() = Loaded {
            settings: new,
            problem: None,
        };
        Ok(())
    }

    /// Fails while the settings file found at startup is unusable.
    pub(crate) fn ready(&self) -> Result<(), AiError> {
        match &self.loaded().problem {
            Some(problem) => Err(AiError::BadSettingsFile(problem.clone())),
            None => Ok(()),
        }
    }

    fn loaded(&self) -> MutexGuard<'_, Loaded> {
        // Nothing panics while holding it, and the values stay whole anyway.
        self.loaded.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
