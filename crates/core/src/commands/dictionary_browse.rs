use dictionary::{DictionaryStats, SenseKey, SensePage};
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

#[tauri::command]
#[specta::specta]
pub async fn dictionary_stats(state: State<'_, AppState>) -> Result<DictionaryStats, CommandError> {
    Ok(state.dictionary.stats().await?)
}

#[tauri::command]
#[specta::specta]
pub async fn search_senses(
    state: State<'_, AppState>,
    query: String,
    offset: u32,
    limit: u32,
) -> Result<SensePage, CommandError> {
    Ok(state.dictionary.search(&query, offset, limit).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn delete_sense(state: State<'_, AppState>, key: SenseKey) -> Result<bool, CommandError> {
    Ok(state.dictionary.delete_sense(&key).await?)
}
