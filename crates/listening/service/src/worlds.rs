//! Worlds and their casts.

use listening_drama::world::store::cast::{add, remove, set_role};
use listening_drama::world::store::{create, delete, detail, list, setting};

use crate::{ListeningError, ListeningService, Role, WorldDetail, WorldSummary};

impl ListeningService {
    /// Every world with its number of episodes.
    pub async fn worlds(&self) -> Result<Vec<WorldSummary>, ListeningError> {
        Ok(list::worlds(&self.db).await?)
    }

    /// One world with its setting and cast; `None` if it is gone.
    pub async fn world(&self, name: &str) -> Result<Option<WorldDetail>, ListeningError> {
        Ok(detail::world(&self.db, name).await?)
    }

    pub async fn create_world(&self, name: &str, setting: &str) -> Result<(), ListeningError> {
        Ok(create::create_world(&self.db, name, setting).await?)
    }

    pub async fn update_setting(&self, name: &str, setting: &str) -> Result<(), ListeningError> {
        Ok(setting::update_setting(&self.db, name, setting).await?)
    }

    /// Puts a character in a world's cast.
    pub async fn add_cast(
        &self,
        world: &str,
        character: &str,
        role: &Role,
    ) -> Result<(), ListeningError> {
        Ok(add::add_cast(&self.db, world, character, role).await?)
    }

    /// Changes what a character is in a world, in place.
    pub async fn set_role(
        &self,
        world: &str,
        character: &str,
        role: &Role,
    ) -> Result<(), ListeningError> {
        Ok(set_role::set_role(&self.db, world, character, role).await?)
    }

    /// Takes a character out of a world's cast. `false` if it was not in it.
    pub async fn remove_cast(&self, world: &str, character: &str) -> Result<bool, ListeningError> {
        Ok(remove::remove_cast(&self.db, world, character).await?)
    }

    /// Deletes a world with its episodes and their audio. `false` if it is
    /// already gone.
    pub async fn delete_world(&self, name: &str) -> Result<bool, ListeningError> {
        Ok(delete::delete_world(&self.db, name).await?)
    }
}
