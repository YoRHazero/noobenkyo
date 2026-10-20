use dictionary::DictionaryStats;
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

#[tauri::command]
#[specta::specta]
pub async fn dictionary_stats(state: State<'_, AppState>) -> Result<DictionaryStats, CommandError> {
    Ok(state.dictionary.stats().await?)
}
