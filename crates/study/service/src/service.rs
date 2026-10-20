use std::path::{Path, PathBuf};

use rand::SeedableRng;
use rand::rngs::StdRng;
use study_learn::LearnService;
use study_review::ReviewService;

use crate::{StudyError, StudySettings, settings_file};

/// Study: the settings, the learning line and the current review session.
/// Touches only its own data directory; senses come in and go out through
/// the caller.
pub struct StudyService {
    pub(crate) dir: PathBuf,
    pub(crate) settings: StudySettings,
    pub(crate) learn: LearnService,
    pub(crate) review: Option<ReviewService>,
}

impl StudyService {
    /// Reads the settings and the learning zone back; no review session.
    pub fn open(dir: &Path) -> Result<Self, StudyError> {
        std::fs::create_dir_all(dir)?;
        let settings = settings_file::load(dir);
        let learn = LearnService::open(
            dir,
            settings.quiz.clone(),
            settings.learn.clone(),
            StdRng::from_os_rng(),
        );
        Ok(Self {
            dir: dir.to_owned(),
            settings,
            learn,
            review: None,
        })
    }

    pub fn settings(&self) -> StudySettings {
        self.settings.clone()
    }

    /// Replaces the settings as a whole; invalid ones are rejected and the old
    /// ones kept. The learning line takes them at once; a review session goes
    /// on with the settings it started with.
    pub fn update_settings(&mut self, new: StudySettings) -> Result<(), StudyError> {
        new.validate()?;
        settings_file::save(&self.dir, &new)?;
        self.learn
            .apply_settings(new.quiz.clone(), new.learn.clone())?;
        self.settings = new;
        Ok(())
    }

    pub fn learn(&mut self) -> &mut LearnService {
        &mut self.learn
    }
}
