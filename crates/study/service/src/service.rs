use std::path::{Path, PathBuf};

use rand::SeedableRng;
use rand::rngs::StdRng;
use study_workspace::Workspace;
use study_workspace::store::workspace_file;

use crate::{Settings, StudyError, settings_file};

/// Study: the two zones, questions and answers. Touches only its own data
/// directory; senses come in and go out through the caller.
pub struct StudyService {
    pub(crate) dir: PathBuf,
    pub(crate) settings: Settings,
    pub(crate) workspace: Workspace,
    pub(crate) rng: StdRng,
}

impl StudyService {
    pub fn open(dir: &Path) -> Result<Self, StudyError> {
        std::fs::create_dir_all(dir)?;
        Ok(Self {
            dir: dir.to_owned(),
            settings: settings_file::load(dir),
            workspace: Workspace::new(workspace_file::load(dir)),
            rng: StdRng::from_os_rng(),
        })
    }

    pub fn settings(&self) -> Settings {
        self.settings.clone()
    }

    /// Replaces the settings as a whole; invalid settings are rejected and the old ones kept.
    pub fn update_settings(&mut self, new: Settings) -> Result<(), StudyError> {
        new.validate()?;
        settings_file::save(&self.dir, &new)?;
        self.settings = new;
        Ok(())
    }

    pub(crate) fn persist(&self) -> Result<(), StudyError> {
        Ok(workspace_file::save(&self.dir, self.workspace.state())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_settings_and_keeps_old() {
        let dir = tempfile::tempdir().unwrap();
        let mut service = StudyService::open(dir.path()).unwrap();
        let mut invalid = Settings::default();
        invalid.quiz.kanji_order.clear();
        assert!(service.update_settings(invalid).is_err());
        assert_eq!(service.settings(), Settings::default());

        let mut valid = Settings::default();
        valid.workspace.review_capacity = 3;
        service.update_settings(valid.clone()).unwrap();
        assert_eq!(StudyService::open(dir.path()).unwrap().settings(), valid);
    }
}
