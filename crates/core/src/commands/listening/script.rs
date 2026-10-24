use listening::{ScriptPreview, Uuid};
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

/// What a script would be stored as, for the script preview, or the problems
/// that stop it. Asynchronous, although pure, because its time grows with
/// the text.
#[tauri::command]
#[specta::specta]
pub async fn listening_preview_script(text: String) -> ScriptPreview {
    listening::preview_script(&text)
}

/// Checks a script and stores it as a new episode of the world. Returns the
/// new episode's uuid.
#[tauri::command]
#[specta::specta]
pub async fn listening_import_script(
    state: State<'_, AppState>,
    world: String,
    text: String,
) -> Result<Uuid, CommandError> {
    Ok(state.listening.import_script(&world, &text).await?)
}

/// Checks a script and continues the episode with it: its lines are
/// appended and its summary replaces the episode's.
#[tauri::command]
#[specta::specta]
pub async fn listening_append_script(
    state: State<'_, AppState>,
    episode: Uuid,
    text: String,
) -> Result<(), CommandError> {
    Ok(state.listening.append_script(episode, &text).await?)
}
