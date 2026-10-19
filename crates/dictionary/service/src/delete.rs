//! Deleting from the dictionary. Each delete takes everything below with
//! it; each returns `false` if there was nothing to delete.

use dictionary_word::store::delete::{reading, sense, sentence, word};

use crate::{DictionaryError, DictionaryService, ReadingKey, SenseKey, WordKey};

impl DictionaryService {
    pub async fn delete_word(&self, key: &WordKey) -> Result<bool, DictionaryError> {
        Ok(word::delete_word(&self.db, key).await?)
    }

    pub async fn delete_reading(&self, key: &ReadingKey) -> Result<bool, DictionaryError> {
        Ok(reading::delete_reading(&self.db, key).await?)
    }

    pub async fn delete_sense(&self, key: &SenseKey) -> Result<bool, DictionaryError> {
        Ok(sense::delete_sense(&self.db, key).await?)
    }

    /// Deletes the sentence of a sense whose plain text equals that of `marked`.
    pub async fn delete_sentence(
        &self,
        key: &SenseKey,
        marked: &str,
    ) -> Result<bool, DictionaryError> {
        Ok(sentence::delete_sentence(&self.db, key, marked).await?)
    }
}
