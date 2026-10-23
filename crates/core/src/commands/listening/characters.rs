use listening::{CharacterDetail, CharacterSummary, Language, NewCharacter, Reference};
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

#[tauri::command]
#[specta::specta]
pub async fn listening_characters(
    state: State<'_, AppState>,
) -> Result<Vec<CharacterSummary>, CommandError> {
    Ok(state.listening.characters().await?)
}

/// One character with its prompt and reference texts; `None` if it is gone.
#[tauri::command]
#[specta::specta]
pub async fn listening_character(
    state: State<'_, AppState>,
    name: String,
) -> Result<Option<CharacterDetail>, CommandError> {
    Ok(state.listening.character(&name).await?)
}

/// Creates a character with the references the user uploaded. Returns the
/// problems that rejected it, empty when it was created.
#[tauri::command]
#[specta::specta]
pub async fn listening_create_character(
    state: State<'_, AppState>,
    character: NewCharacter,
) -> Result<Vec<String>, CommandError> {
    Ok(state.listening.create_character(&character).await?)
}

/// Adds the reference recording a character lacks in `language`.
#[tauri::command]
#[specta::specta]
pub async fn listening_add_reference(
    state: State<'_, AppState>,
    name: String,
    language: Language,
    audio: Vec<u8>,
    text: String,
) -> Result<(), CommandError> {
    Ok(state
        .listening
        .add_reference(&name, language, &audio, &text)
        .await?)
}

/// A character's reference recording in `language`, to listen to; `None`
/// if it has none.
#[tauri::command]
#[specta::specta]
pub async fn listening_reference(
    state: State<'_, AppState>,
    name: String,
    language: Language,
) -> Result<Option<Reference>, CommandError> {
    Ok(state.listening.reference(&name, language).await?)
}

/// Names of the worlds whose cast has the character, to confirm a delete.
#[tauri::command]
#[specta::specta]
pub async fn listening_character_usage(
    state: State<'_, AppState>,
    name: String,
) -> Result<Vec<String>, CommandError> {
    Ok(state.listening.character_usage(&name).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn listening_delete_character(
    state: State<'_, AppState>,
    name: String,
) -> Result<bool, CommandError> {
    Ok(state.listening.delete_character(&name).await?)
}
