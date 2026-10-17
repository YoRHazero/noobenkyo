//! Data exchanged with study when senses enter and leave zones.

use dictionary_word::Progress;
use dictionary_word::store::{candidates, progress, sentence_removal, snapshot};

use crate::{DictionaryError, DictionaryService, SenseCandidate, SenseKey, SenseSnapshot, clock};

impl DictionaryService {
    /// Senses with at least one sentence, in creation order.
    pub async fn candidates(&self) -> Result<Vec<SenseCandidate>, DictionaryError> {
        Ok(candidates::list_candidates(&self.db).await?)
    }

    /// Snapshots of the given senses; senses that no longer exist are left out.
    pub async fn snapshots(
        &self,
        keys: &[SenseKey],
    ) -> Result<Vec<SenseSnapshot>, DictionaryError> {
        let mut snapshots = Vec::with_capacity(keys.len());
        for key in keys {
            if let Some(found) = snapshot::load_snapshot(&self.db, key).await? {
                snapshots.push(found);
            }
        }
        Ok(snapshots)
    }

    /// Overwrites a sense's progress and stamps it now. Returns `false` if the sense is gone.
    pub async fn settle(
        &self,
        key: &SenseKey,
        stage: u32,
        passes: u32,
        lapses: u32,
    ) -> Result<bool, DictionaryError> {
        let written = Progress {
            stage,
            passes,
            lapses,
            modified_at: clock::now(),
        };
        Ok(progress::write_progress(&self.db, key, &written).await?)
    }

    /// Deletes a reported sentence. Returns `false` if it is already gone.
    pub async fn remove_sentence(
        &self,
        key: &SenseKey,
        marked: &str,
    ) -> Result<bool, DictionaryError> {
        Ok(sentence_removal::remove_sentence(&self.db, key, marked).await?)
    }
}
