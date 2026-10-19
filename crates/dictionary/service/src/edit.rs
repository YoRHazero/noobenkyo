//! Changing a sense in place: its content, or its progress.

use dictionary_word::Progress;
use dictionary_word::store::edit::{replace_sense, update_progress};

use crate::{DictionaryError, DictionaryService, Replaced, SenseKey, SensePatch, clock};

impl DictionaryService {
    /// Replaces the given fields of a sense; any problem leaves it untouched.
    pub async fn replace_sense(
        &self,
        key: &SenseKey,
        patch: &SensePatch,
    ) -> Result<Replaced, DictionaryError> {
        Ok(replace_sense::replace_sense(&self.db, key, patch).await?)
    }

    /// Overwrites the progress of a sense and stamps it now. `false` if the sense is gone.
    pub async fn update_progress(
        &self,
        key: &SenseKey,
        stage: u32,
        passes: u32,
        lapses: u32,
    ) -> Result<bool, DictionaryError> {
        let progress = Progress {
            stage,
            passes,
            lapses,
            modified_at: clock::now(),
        };
        Ok(update_progress::update_progress(&self.db, key, &progress).await?)
    }
}
