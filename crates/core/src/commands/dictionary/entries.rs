use dictionary::{ContentFilter, ImportOptions, ImportReport};
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;
use crate::user_files;

/// What importing the file would drop or ignore, one line of Chinese each.
#[tauri::command]
#[specta::specta]
pub async fn dictionary_check_entries(path: String) -> Result<Vec<String>, CommandError> {
    let text = user_files::read_text(&path)?;
    Ok(dictionary::check_entries(&text))
}

#[tauri::command]
#[specta::specta]
pub async fn dictionary_import(
    state: State<'_, AppState>,
    path: String,
    options: ImportOptions,
) -> Result<ImportReport, CommandError> {
    let text = user_files::read_text(&path)?;
    Ok(state.dictionary.import(&text, options).await?)
}

/// The options the import dialog opens with.
#[tauri::command]
#[specta::specta]
pub fn dictionary_default_import_options() -> ImportOptions {
    ImportOptions::default()
}

/// Writes the matching rows to the file; returns how many lines were written.
#[tauri::command]
#[specta::specta]
pub async fn dictionary_export(
    state: State<'_, AppState>,
    path: String,
    filter: ContentFilter,
    limit: Option<u32>,
    keep_progress: bool,
) -> Result<u32, CommandError> {
    let exported = state
        .dictionary
        .export(&filter, limit, keep_progress)
        .await?;
    user_files::write_text(&path, &exported.text)?;
    Ok(exported.entries)
}
