//! Episodes of a world.

use listening_drama::episode::store::{delete, detail, list, update};

use crate::{EpisodeDetail, EpisodeSummary, ListeningError, ListeningService, Uuid};

impl ListeningService {
    /// A world's episodes by number; equal numbers in the order they were created.
    pub async fn episodes(&self, world: &str) -> Result<Vec<EpisodeSummary>, ListeningError> {
        Ok(list::episodes(&self.db, world).await?)
    }

    /// One episode with its lines; `None` if it is gone.
    pub async fn episode(&self, episode: Uuid) -> Result<Option<EpisodeDetail>, ListeningError> {
        Ok(detail::episode(&self.db, episode).await?)
    }

    /// Replaces the number, title and summary together. Returns the problems
    /// that rejected them, empty when they were replaced.
    pub async fn update_episode(
        &self,
        episode: Uuid,
        number: f64,
        title: &str,
        summary: &str,
    ) -> Result<Vec<String>, ListeningError> {
        Ok(update::update_episode(&self.db, episode, number, title, summary).await?)
    }

    /// Deletes an episode with its lines and audio. `false` if it is already gone.
    pub async fn delete_episode(&self, episode: Uuid) -> Result<bool, ListeningError> {
        Ok(delete::delete_episode(&self.db, episode).await?)
    }
}
