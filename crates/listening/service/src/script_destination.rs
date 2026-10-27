//! Whether a script could be stored where it is meant to go, checked before
//! it is stored: the script check, and whether the world or the episode can
//! take it now. What an agent sends, and the edits written back from the
//! script preview, pass this before they wait in the inbox.

use crate::{ListeningError, ListeningService, Uuid, check_script};

impl ListeningService {
    /// Why the script cannot become a new episode of the world now: the
    /// script check, then whether the world is there with a narrator and a
    /// main character. Empty when it can.
    pub async fn check_import_script(
        &self,
        world: &str,
        text: &str,
    ) -> Result<Vec<String>, ListeningError> {
        let mut problems = check_script(text);
        problems.extend(self.cast_problems(world, None).await?);
        Ok(problems)
    }

    /// Why the script cannot continue the episode now: the script check,
    /// then whether the episode is there, in a world with a narrator and a
    /// main character. Empty when it can.
    pub async fn check_append_script(
        &self,
        episode: Uuid,
        text: &str,
    ) -> Result<Vec<String>, ListeningError> {
        let mut problems = check_script(text);
        match self.episode(episode).await? {
            Some(found) => problems.extend(self.cast_problems(&found.world, Some(episode)).await?),
            None => problems.push(format!("要续写的集 {episode} 不存在")),
        }
        Ok(problems)
    }

    /// Why the world cannot take a script now; empty when it can.
    async fn cast_problems(
        &self,
        world: &str,
        appending: Option<Uuid>,
    ) -> Result<Vec<String>, ListeningError> {
        match self.script_context(world, &[], appending).await {
            Ok(_) => Ok(Vec::new()),
            // The drama's own errors say what is missing: the world, the
            // narrator, a main character.
            Err(ListeningError::Drama(error)) => Ok(vec![error.to_string()]),
            Err(error) => Err(error),
        }
    }
}
