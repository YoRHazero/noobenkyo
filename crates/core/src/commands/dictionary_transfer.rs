use dictionary::{ImportOptions, ImportReport};
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;
use crate::user_files;

/// Imports the entry file at `path`.
#[tauri::command]
#[specta::specta]
pub async fn import_entries(
    state: State<'_, AppState>,
    path: String,
    options: ImportOptions,
) -> Result<ImportReport, CommandError> {
    let text = user_files::read_text(&path)?;
    Ok(state.dictionary.import(&text, options).await?)
}

/// Writes the dictionary to `path` as entries; returns how many were written.
#[tauri::command]
#[specta::specta]
pub async fn export_entries(
    state: State<'_, AppState>,
    path: String,
    keep_progress: bool,
) -> Result<u32, CommandError> {
    let exported = state.dictionary.export(keep_progress).await?;
    user_files::write_text(&path, &exported.text)?;
    Ok(exported.entries)
}
