//! Study settings: read, replace as a whole, and the defaults to show and to
//! restore from.

use study::StudySettings;
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

#[tauri::command]
#[specta::specta]
pub async fn study_settings(state: State<'_, AppState>) -> Result<StudySettings, CommandError> {
    Ok(state.study.lock().await.settings())
}

/// Invalid settings are rejected and the old ones kept.
#[tauri::command]
#[specta::specta]
pub async fn update_study_settings(
    state: State<'_, AppState>,
    settings: StudySettings,
) -> Result<(), CommandError> {
    Ok(state.study.lock().await.update_settings(settings)?)
}

#[tauri::command]
#[specta::specta]
pub fn study_default_settings() -> StudySettings {
    StudySettings::default()
}
