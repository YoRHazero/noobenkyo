use dictionary::{Replaced, SenseKey, SensePatch};
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

#[tauri::command]
#[specta::specta]
pub async fn dictionary_replace_sense(
    state: State<'_, AppState>,
    key: SenseKey,
    patch: SensePatch,
) -> Result<Replaced, CommandError> {
    Ok(state.dictionary.replace_sense(&key, &patch).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn dictionary_update_progress(
    state: State<'_, AppState>,
    key: SenseKey,
    stage: u32,
    passes: u32,
    lapses: u32,
) -> Result<bool, CommandError> {
    Ok(state
        .dictionary
        .update_progress(&key, stage, passes, lapses)
        .await?)
}
