use inbox::InboxSource;
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

/// The entry text waiting, for the import dialog; it leaves the inbox.
#[tauri::command]
#[specta::specta]
pub async fn inbox_take_entries(
    state: State<'_, AppState>,
    source: InboxSource,
    name: String,
) -> Result<String, CommandError> {
    Ok(state.inbox.take_entries(source, &name)?)
}
