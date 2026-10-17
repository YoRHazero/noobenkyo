use study::Settings;
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

#[tauri::command]
#[specta::specta]
pub async fn study_settings(state: State<'_, AppState>) -> Result<Settings, CommandError> {
    Ok(state.study.lock().await.settings())
}

#[tauri::command]
#[specta::specta]
pub async fn update_study_settings(
    state: State<'_, AppState>,
    settings: Settings,
) -> Result<(), CommandError> {
    Ok(state.study.lock().await.update_settings(settings)?)
}

#[tauri::command]
#[specta::specta]
pub async fn reset_study_settings(state: State<'_, AppState>) -> Result<Settings, CommandError> {
    let mut study = state.study.lock().await;
    study.update_settings(Settings::default())?;
    Ok(study.settings())
}
