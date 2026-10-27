//! Sending what an agent wrote to the app through the inbox, into the
//! agent's own folder. The inbox only goes one way, so everything that can
//! be checked here is checked before anything is sent: the agent never sees
//! what the app finds later. The checks are listening's, the same ones the
//! app runs on edits written back from the preview.

use inbox::{Delivery, InboxError, InboxService, InboxSource, ScriptTarget};
use listening::{ListeningError, ListeningService};
use thiserror::Error;

/// Why a delivery was not sent.
#[derive(Debug, Error)]
pub enum SendError {
    /// What the agent has to fix, one line of Chinese each.
    #[error("{}", .0.join("\n"))]
    Problems(Vec<String>),
    #[error(transparent)]
    Listening(ListeningError),
    #[error(transparent)]
    Inbox(#[from] InboxError),
}

/// Sends entries for the import dialog under `name`. Warnings do not stop
/// them: the dialog shows them, and lines a model cannot fix are still worth
/// importing.
pub fn send_entries(inbox: &InboxService, name: &str, text: &str) -> Result<(), InboxError> {
    inbox.put(
        InboxSource::Agent,
        name,
        &Delivery::Entries {
            text: text.to_owned(),
        },
    )
}

/// Sends a new script for the script preview under `name`, once it passes
/// the check and its target can take it: a new episode needs the world with
/// a narrator and a main character; continuing needs the episode, in such a
/// world.
pub async fn send_script(
    listening: &ListeningService,
    inbox: &InboxService,
    name: &str,
    text: &str,
    target: ScriptTarget,
) -> Result<(), SendError> {
    let problems = script_problems(listening, text, &target).await?;
    if !problems.is_empty() {
        return Err(SendError::Problems(problems));
    }
    inbox.put(
        InboxSource::Agent,
        name,
        &Delivery::Script {
            target,
            text: text.to_owned(),
        },
    )?;
    Ok(())
}

/// Replaces the script waiting under `name` as a whole with the agent's new
/// version, after the same checks as sending. Only a script still waiting
/// can be replaced, and that is said before any check.
pub async fn replace_script(
    listening: &ListeningService,
    inbox: &InboxService,
    name: &str,
    text: &str,
    target: ScriptTarget,
) -> Result<(), SendError> {
    inbox.require_script(InboxSource::Agent, name)?;
    let problems = script_problems(listening, text, &target).await?;
    if !problems.is_empty() {
        return Err(SendError::Problems(problems));
    }
    inbox.replace_script(InboxSource::Agent, name, target, text)?;
    Ok(())
}

/// Why the script cannot go to its target now; empty when it can.
async fn script_problems(
    listening: &ListeningService,
    text: &str,
    target: &ScriptTarget,
) -> Result<Vec<String>, SendError> {
    let checked = match target {
        ScriptTarget::NewEpisode { world } => listening.check_import_script(world, text).await,
        ScriptTarget::Append { episode } => listening.check_append_script(*episode, text).await,
    };
    checked.map_err(SendError::Listening)
}
