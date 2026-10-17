use dictionary::Settings;
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

#[tauri::command]
#[specta::specta]
pub fn dictionary_settings(state: State<'_, AppState>) -> Settings {
    state.dictionary.settings()
}

#[tauri::command]
#[specta::specta]
pub fn update_dictionary_settings(
    state: State<'_, AppState>,
    settings: Settings,
) -> Result<(), CommandError> {
    Ok(state.dictionary.update_settings(settings)?)
}

#[tauri::command]
#[specta::specta]
pub fn reset_dictionary_settings(state: State<'_, AppState>) -> Result<Settings, CommandError> {
    state.dictionary.update_settings(Settings::default())?;
    Ok(state.dictionary.settings())
}
