use std::path::{Path, PathBuf};

use rand::rngs::StdRng;
use study_quiz::{Pending, QuizSettings, Recent, SenseKey};

use crate::rules::clears::clears_zone;
use crate::store::learning_file;
use crate::{Item, LearnError, LearnSettings};

/// The learning zone. Its items live in learning.json; the quiz in progress
/// and the recently asked senses live in memory only.
pub struct LearnService {
    pub(crate) dir: PathBuf,
    pub(crate) quiz: QuizSettings,
    pub(crate) settings: LearnSettings,
    pub(crate) items: Vec<Item>,
    /// The sense `next` chose, waiting for its content.
    pub(crate) chosen: Option<SenseKey>,
    pub(crate) pending: Option<Pending>,
    pub(crate) recent: Recent,
    pub(crate) rng: StdRng,
}

impl LearnService {
    /// Reads the zone back from `dir`; the settings come from the caller.
    pub fn open(dir: &Path, quiz: QuizSettings, settings: LearnSettings, rng: StdRng) -> Self {
        Self {
            dir: dir.to_owned(),
            quiz,
            settings,
            items: learning_file::load(dir),
            chosen: None,
            pending: None,
            recent: Recent::default(),
            rng,
        }
    }

    /// Takes new settings. A changed capacity or question order empties the
    /// zone without writing anything back.
    pub fn apply_settings(
        &mut self,
        quiz: QuizSettings,
        settings: LearnSettings,
    ) -> Result<(), LearnError> {
        let clears = clears_zone((&self.quiz, &self.settings), (&quiz, &settings));
        self.quiz = quiz;
        self.settings = settings;
        if clears {
            self.items.clear();
            self.chosen = None;
            self.pending = None;
            self.recent.clear();
            self.save()?;
        }
        Ok(())
    }

    pub(crate) fn save(&self) -> Result<(), LearnError> {
        Ok(learning_file::save(&self.dir, &self.items)?)
    }

    pub(crate) fn position(&self, key: &SenseKey) -> Option<usize> {
        self.items.iter().position(|item| &item.key == key)
    }
}
