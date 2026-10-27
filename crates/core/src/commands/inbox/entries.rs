use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

/// The entry text waiting, for the import dialog; it leaves the inbox.
#[tauri::command]
#[specta::specta]
pub async fn inbox_take_entries(
    state: State<'_, AppState>,
    name: String,
) -> Result<String, CommandError> {
    Ok(state.agent.take_entries(&name)?)
}
