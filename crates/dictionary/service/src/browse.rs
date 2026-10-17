//! Browsing the dictionary and deleting senses.

use dictionary_word::store::{browse, sense, sense_by_key, stats};

use crate::{DictionaryError, DictionaryService, DictionaryStats, SenseKey, SensePage};

impl DictionaryService {
    pub async fn stats(&self) -> Result<DictionaryStats, DictionaryError> {
        Ok(stats::count_stats(&self.db).await?)
    }

    pub async fn search(
        &self,
        query: &str,
        offset: u32,
        limit: u32,
    ) -> Result<SensePage, DictionaryError> {
        Ok(browse::search_senses(&self.db, query, offset, limit).await?)
    }

    /// Deletes a sense with its labels and sentences. Returns `false` if it is already gone.
    pub async fn delete_sense(&self, key: &SenseKey) -> Result<bool, DictionaryError> {
        let Some(found) = sense_by_key::find_sense(&self.db, key).await? else {
            return Ok(false);
        };
        sense::delete_sense(&self.db, found.id).await?;
        Ok(true)
    }
}
