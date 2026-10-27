use inbox::{InboxScript, InboxSource, ScriptTarget};
use listening::{ListeningError, ListeningService, Uuid};
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

/// The script waiting and where it goes; `null` once it is saved or
/// discarded. Leaves the file as it is.
#[tauri::command]
#[specta::specta]
pub async fn inbox_read_script(
    state: State<'_, AppState>,
    source: InboxSource,
    name: String,
) -> Result<Option<InboxScript>, CommandError> {
    Ok(state.inbox.read_script(source, &name)?)
}

/// Writes an edit from the preview back to the inbox, once listening finds
/// no problem with it and where it goes. Returns the problems that kept it
/// out, empty when it was written.
#[tauri::command]
#[specta::specta]
pub async fn inbox_replace_script(
    state: State<'_, AppState>,
    source: InboxSource,
    name: String,
    target: ScriptTarget,
    text: String,
) -> Result<Vec<String>, CommandError> {
    state.inbox.require_script(source, &name)?;
    let problems = script_problems(&state.listening, &target, &text).await?;
    if problems.is_empty() {
        state.inbox.replace_script(source, &name, target, &text)?;
    }
    Ok(problems)
}

/// Stores the script shown as a new episode or a continuation, then removes
/// it from the inbox. Returns the episode's uuid: the new one, or the one
/// continued.
#[tauri::command]
#[specta::specta]
pub async fn inbox_save_script(
    state: State<'_, AppState>,
    source: InboxSource,
    name: String,
    target: ScriptTarget,
    text: String,
) -> Result<Uuid, CommandError> {
    // A script already saved or discarded is not stored again.
    state.inbox.require_script(source, &name)?;
    let episode = match target {
        ScriptTarget::NewEpisode { world } => state.listening.import_script(&world, &text).await?,
        ScriptTarget::Append { episode } => {
            state.listening.append_script(episode, &text).await?;
            episode
        }
    };
    state.inbox.remove_saved_script(source, &name)?;
    Ok(episode)
}

#[tauri::command]
#[specta::specta]
pub async fn inbox_discard_script(
    state: State<'_, AppState>,
    source: InboxSource,
    name: String,
) -> Result<(), CommandError> {
    Ok(state.inbox.discard_script(source, &name)?)
}

/// Listening's check of the script for where it goes: one call per kind of
/// target.
async fn script_problems(
    listening: &ListeningService,
    target: &ScriptTarget,
    text: &str,
) -> Result<Vec<String>, ListeningError> {
    match target {
        ScriptTarget::NewEpisode { world } => listening.check_import_script(world, text).await,
        ScriptTarget::Append { episode } => listening.check_append_script(*episode, text).await,
    }
}
