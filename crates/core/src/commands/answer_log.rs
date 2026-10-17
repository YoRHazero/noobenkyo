use std::path::PathBuf;

use dictionary::LogImportReport;
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

#[tauri::command]
#[specta::specta]
pub async fn log_count(state: State<'_, AppState>) -> Result<u32, CommandError> {
    Ok(state.dictionary.log_count().await?)
}

#[tauri::command]
#[specta::specta]
pub async fn import_log(
    state: State<'_, AppState>,
    path: String,
) -> Result<LogImportReport, CommandError> {
    Ok(state.dictionary.import_log(&PathBuf::from(path)).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn export_log(state: State<'_, AppState>, path: String) -> Result<u32, CommandError> {
    Ok(state.dictionary.export_log(&PathBuf::from(path)).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn clear_log(state: State<'_, AppState>) -> Result<(), CommandError> {
    Ok(state.dictionary.clear_log().await?)
}
