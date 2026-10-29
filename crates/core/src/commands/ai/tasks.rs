use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;
use crate::tasks::TaskId;

/// Stops a task; it ends as cancelled, and what it stored stays. A task that
/// has already ended is left alone.
#[tauri::command]
#[specta::specta]
pub async fn ai_cancel(state: State<'_, AppState>, task: TaskId) -> Result<(), CommandError> {
    state.tasks.cancel(task);
    Ok(())
}
