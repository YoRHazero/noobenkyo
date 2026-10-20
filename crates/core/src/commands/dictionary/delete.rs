use dictionary::{ReadingKey, SenseKey, WordKey};
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

#[tauri::command]
#[specta::specta]
pub async fn dictionary_delete_word(
    state: State<'_, AppState>,
    key: WordKey,
) -> Result<bool, CommandError> {
    Ok(state.dictionary.delete_word(&key).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn dictionary_delete_reading(
    state: State<'_, AppState>,
    key: ReadingKey,
) -> Result<bool, CommandError> {
    Ok(state.dictionary.delete_reading(&key).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn dictionary_delete_sense(
    state: State<'_, AppState>,
    key: SenseKey,
) -> Result<bool, CommandError> {
    Ok(state.dictionary.delete_sense(&key).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn dictionary_delete_sentence(
    state: State<'_, AppState>,
    key: SenseKey,
    marked: String,
) -> Result<bool, CommandError> {
    Ok(state.dictionary.delete_sentence(&key, &marked).await?)
}
