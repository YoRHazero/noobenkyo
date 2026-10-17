use std::path::PathBuf;

use dictionary::{ExportReport, ImportOptions, ImportReport};
use tauri::{AppHandle, State};
use tauri_specta::Event;

use crate::app_state::AppState;
use crate::command_error::CommandError;
use crate::events::ImportProgressEvent;

/// Imports an entry file, pushing progress events while it runs.
#[tauri::command]
#[specta::specta]
pub async fn import_entries(
    app: AppHandle,
    state: State<'_, AppState>,
    path: String,
    options: ImportOptions,
) -> Result<ImportReport, CommandError> {
    let mut on_progress = |progress| {
        let _ = ImportProgressEvent(progress).emit(&app);
    };
    Ok(state
        .dictionary
        .import_entries(&PathBuf::from(path), options, &mut on_progress)
        .await?)
}

#[tauri::command]
#[specta::specta]
pub async fn export_entries(
    state: State<'_, AppState>,
    path: String,
    keep_progress: bool,
) -> Result<ExportReport, CommandError> {
    Ok(state
        .dictionary
        .export_entries(&PathBuf::from(path), keep_progress)
        .await?)
}
