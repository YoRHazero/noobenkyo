//! The ai group's settings for the model page: read, replace as a whole,
//! and the defaults.

use ai::AiSettings;
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

#[tauri::command]
#[specta::specta]
pub fn ai_settings(state: State<'_, AppState>) -> AiSettings {
    state.ai.settings()
}

/// Invalid settings are rejected and the old ones kept; so are any while a
/// model call runs or waits.
#[tauri::command]
#[specta::specta]
pub fn ai_update_settings(
    state: State<'_, AppState>,
    settings: AiSettings,
) -> Result<(), CommandError> {
    Ok(state.ai.update_settings(settings)?)
}

#[tauri::command]
#[specta::specta]
pub fn ai_default_settings() -> AiSettings {
    AiSettings::default()
}
