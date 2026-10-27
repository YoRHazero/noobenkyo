//! Sending what an agent wrote to the app through the inbox. The inbox only
//! goes one way, so everything that can be checked here is checked before
//! anything is sent: the agent never sees what the app finds later. Edits
//! the app writes back to a script pass the same check.

use listening::{ListeningError, ListeningService, check_script};
use thiserror::Error;

use crate::{Delivery, Inbox, InboxError, ScriptTarget};

/// Why a delivery was not sent.
#[derive(Debug, Error)]
pub enum SendError {
    /// What the agent has to fix, one line of Chinese each.
    #[error("{}", .0.join("\n"))]
    Problems(Vec<String>),
    #[error("inbox 里的 {0} 不是剧本")]
    NotScript(String),
    #[error(transparent)]
    Listening(ListeningError),
    #[error(transparent)]
    Inbox(#[from] InboxError),
}

/// Sends entries for the import dialog under `name`. Warnings do not stop
/// them: the dialog shows them, and lines a model cannot fix are still worth
/// importing.
pub fn send_entries(inbox: &Inbox, name: &str, text: &str) -> Result<(), InboxError> {
    inbox.put(
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
    inbox: &Inbox,
    name: &str,
    text: &str,
    target: ScriptTarget,
) -> Result<(), SendError> {
    let delivery = checked_script(listening, text, target).await?;
    inbox.put(name, &delivery)?;
    Ok(())
}

/// Replaces the script waiting under `name` as a whole, after the same
/// checks as sending: an agent's new version, or the edits made in its
/// preview.
pub async fn replace_script(
    listening: &ListeningService,
    inbox: &Inbox,
    name: &str,
    text: &str,
    target: ScriptTarget,
) -> Result<(), SendError> {
    match inbox.read(name)? {
        Some(Delivery::Script { .. }) => {}
        Some(Delivery::Entries { .. }) => return Err(SendError::NotScript(name.to_owned())),
        None => return Err(InboxError::Missing(name.to_owned()).into()),
    }
    let delivery = checked_script(listening, text, target).await?;
    inbox.replace(name, &delivery)?;
    Ok(())
}

/// The script as a delivery, once it passes the check and its target can
/// take it.
async fn checked_script(
    listening: &ListeningService,
    text: &str,
    target: ScriptTarget,
) -> Result<Delivery, SendError> {
    let mut problems = check_script(text);
    problems.extend(target_problems(listening, &target).await?);
    if !problems.is_empty() {
        return Err(SendError::Problems(problems));
    }
    Ok(Delivery::Script {
        target,
        text: text.to_owned(),
    })
}

/// Why the target cannot take a script now; empty when it can.
async fn target_problems(
    listening: &ListeningService,
    target: &ScriptTarget,
) -> Result<Vec<String>, SendError> {
    let (world, appending) = match target {
        ScriptTarget::NewEpisode { world } => (world.clone(), None),
        ScriptTarget::Append { episode } => match listening.episode(*episode).await {
            Ok(Some(found)) => (found.world, Some(*episode)),
            Ok(None) => return Ok(vec![format!("要续写的集 {episode} 不存在")]),
            Err(error) => return Err(SendError::Listening(error)),
        },
    };
    match listening.script_context(&world, &[], appending).await {
        Ok(_) => Ok(Vec::new()),
        // The drama's own errors say what is missing: the world, the narrator,
        // a main character.
        Err(ListeningError::Drama(error)) => Ok(vec![error.to_string()]),
        Err(error) => Err(SendError::Listening(error)),
    }
}
