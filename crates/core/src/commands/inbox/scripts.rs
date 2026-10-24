use ai::{InboxScript, ScriptTarget};
use listening::Uuid;
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

/// The script waiting and where it goes; `null` once it is saved or
/// discarded. Leaves the file as it is.
#[tauri::command]
#[specta::specta]
pub async fn inbox_read_script(
    state: State<'_, AppState>,
    name: String,
) -> Result<Option<InboxScript>, CommandError> {
    Ok(state.ai.read_script(&name)?)
}

/// Writes an edit from the preview back to the inbox, if it passes. Returns
/// the problems that kept it out, empty when it was written.
#[tauri::command]
#[specta::specta]
pub async fn inbox_replace_script(
    state: State<'_, AppState>,
    name: String,
    target: ScriptTarget,
    text: String,
) -> Result<Vec<String>, CommandError> {
    Ok(state
        .ai
        .replace_script(&state.listening, &name, target, &text)
        .await?)
}

/// Stores the script shown as a new episode or a continuation, then removes
/// it from the inbox. Returns the episode's uuid.
#[tauri::command]
#[specta::specta]
pub async fn inbox_save_script(
    state: State<'_, AppState>,
    name: String,
    target: ScriptTarget,
    text: String,
) -> Result<Uuid, CommandError> {
    Ok(state
        .ai
        .save_script(&state.listening, &name, target, &text)
        .await?)
}

#[tauri::command]
#[specta::specta]
pub async fn inbox_discard_script(
    state: State<'_, AppState>,
    name: String,
) -> Result<(), CommandError> {
    Ok(state.ai.discard_script(&name)?)
}
