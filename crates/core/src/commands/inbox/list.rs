use inbox::InboxItem;
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

/// Every item waiting from every source, oldest first, with its kind and
/// source; scripts with where they go. Leaves the files as they are.
#[tauri::command]
#[specta::specta]
pub async fn inbox_list(state: State<'_, AppState>) -> Result<Vec<InboxItem>, CommandError> {
    Ok(state.inbox.list()?)
}
