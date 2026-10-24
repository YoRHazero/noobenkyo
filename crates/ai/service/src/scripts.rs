//! A script stays in the inbox until it is saved or discarded; edits in its
//! preview are written back to it, so the agent reads them with `show`.
//! Writing back and saving use the target and text the caller holds, not
//! the file: what is stored is what the user sees.

use ai_tools::{Delivery, InboxError, ScriptTarget, SendError};
use listening::{ListeningService, Uuid};
use serde::Serialize;

use crate::{AiError, AiService};

/// A script waiting in the inbox: where it goes and its text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct InboxScript {
    pub target: ScriptTarget,
    pub text: String,
}

impl AiService {
    /// The script waiting under `name`; `None` when it is no longer there.
    /// Leaves the file as it is.
    pub fn read_script(&self, name: &str) -> Result<Option<InboxScript>, AiError> {
        match self.inbox.read(name)? {
            Some(Delivery::Script { target, text }) => Ok(Some(InboxScript { target, text })),
            Some(Delivery::Entries { .. }) => Err(AiError::NotScript(name.to_owned())),
            None => Ok(None),
        }
    }

    /// Writes the edited script back under `name`, once it passes the same
    /// checks as sending. Returns the problems that kept it out, empty when
    /// it was written.
    pub async fn replace_script(
        &self,
        listening: &ListeningService,
        name: &str,
        target: ScriptTarget,
        text: &str,
    ) -> Result<Vec<String>, AiError> {
        match ai_tools::replace_script(listening, &self.inbox, name, text, target).await {
            Ok(()) => Ok(Vec::new()),
            Err(SendError::Problems(problems)) => Ok(problems),
            Err(SendError::NotScript(name)) => Err(AiError::NotScript(name)),
            Err(SendError::Listening(error)) => Err(error.into()),
            Err(SendError::Inbox(error)) => Err(error.into()),
        }
    }

    /// Stores the script as a new episode of its world, or appends it to its
    /// episode, then removes it from the inbox. Returns the episode's uuid:
    /// the new one, or the one continued.
    pub async fn save_script(
        &self,
        listening: &ListeningService,
        name: &str,
        target: ScriptTarget,
        text: &str,
    ) -> Result<Uuid, AiError> {
        // A script already saved or discarded is not stored again.
        if self.read_script(name)?.is_none() {
            return Err(InboxError::Missing(name.to_owned()).into());
        }
        let episode = match target {
            ScriptTarget::NewEpisode { world } => listening.import_script(&world, text).await?,
            ScriptTarget::Append { episode } => {
                listening.append_script(episode, text).await?;
                episode
            }
        };
        self.inbox
            .remove(name)
            .map_err(|reason| AiError::SavedButKept {
                name: name.to_owned(),
                reason,
            })?;
        Ok(episode)
    }

    /// Removes the script waiting under `name`. Nothing happens when it is
    /// already gone.
    pub fn discard_script(&self, name: &str) -> Result<(), AiError> {
        if self.read_script(name)?.is_some() {
            self.inbox.remove(name)?;
        }
        Ok(())
    }
}
