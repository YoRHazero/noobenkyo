//! Entry texts in and out. The frontend reads and writes the files the user
//! picks; these commands only take and give text.

use dictionary::{ContentFilter, Exported, ImportOptions, ImportPreview, ImportReport};
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

/// What importing the text would bring in, line by line. Asynchronous,
/// although pure, because its time grows with the text.
#[tauri::command]
#[specta::specta]
pub async fn dictionary_preview_entries(text: String, options: ImportOptions) -> ImportPreview {
    dictionary::preview_entries(&text, options)
}

/// The preview's problems, one line of Chinese each.
#[tauri::command]
#[specta::specta]
pub async fn dictionary_check_entries(text: String, options: ImportOptions) -> Vec<String> {
    dictionary::check_entries(&text, options)
}

#[tauri::command]
#[specta::specta]
pub async fn dictionary_import(
    state: State<'_, AppState>,
    text: String,
    options: ImportOptions,
) -> Result<ImportReport, CommandError> {
    Ok(state.dictionary.import(&text, options).await?)
}

/// The options the import dialog opens with.
#[tauri::command]
#[specta::specta]
pub fn dictionary_default_import_options() -> ImportOptions {
    ImportOptions::default()
}

/// The matching rows as an entry text, for the frontend to write to the file
/// the user picks.
#[tauri::command]
#[specta::specta]
pub async fn dictionary_export(
    state: State<'_, AppState>,
    filter: ContentFilter,
    limit: Option<u32>,
    keep_progress: bool,
) -> Result<Exported, CommandError> {
    Ok(state
        .dictionary
        .export(&filter, limit, keep_progress)
        .await?)
}
