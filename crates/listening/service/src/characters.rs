//! Characters and their reference recordings.

use listening_drama::character::store::{create, delete, detail, list, reference};
use listening_drama::world::store::cast::usage;

use crate::{
    CharacterDetail, CharacterSummary, Language, ListeningError, ListeningService, NewCharacter,
    Reference,
};

impl ListeningService {
    /// Every character, in the order they were created.
    pub async fn characters(&self) -> Result<Vec<CharacterSummary>, ListeningError> {
        Ok(list::characters(&self.db).await?)
    }

    /// One character with its prompt and reference texts; `None` if it is gone.
    pub async fn character(&self, name: &str) -> Result<Option<CharacterDetail>, ListeningError> {
        Ok(detail::character(&self.db, name).await?)
    }

    /// Creates a character. Returns the problems that rejected it, empty
    /// when it was created.
    pub async fn create_character(
        &self,
        new: &NewCharacter,
    ) -> Result<Vec<String>, ListeningError> {
        Ok(create::create_character(&self.db, new).await?)
    }

    /// Adds the reference recording a character lacks in `language`.
    pub async fn add_reference(
        &self,
        name: &str,
        language: Language,
        audio: &[u8],
        text: &str,
    ) -> Result<(), ListeningError> {
        Ok(reference::add_reference(&self.db, name, language, audio, text).await?)
    }

    /// A character's reference recording in `language`; `None` if it has none.
    pub async fn reference(
        &self,
        name: &str,
        language: Language,
    ) -> Result<Option<Reference>, ListeningError> {
        Ok(reference::reference(&self.db, name, language).await?)
    }

    /// Names of the worlds whose cast has the character.
    pub async fn character_usage(&self, name: &str) -> Result<Vec<String>, ListeningError> {
        Ok(usage::character_usage(&self.db, name).await?)
    }

    /// Deletes a character and its places in every cast; episodes keep its
    /// lines. `false` if it is already gone.
    pub async fn delete_character(&self, name: &str) -> Result<bool, ListeningError> {
        Ok(delete::delete_character(&self.db, name).await?)
    }
}
