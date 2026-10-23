use listening::{Role, WorldDetail, WorldSummary};
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

#[tauri::command]
#[specta::specta]
pub async fn listening_worlds(
    state: State<'_, AppState>,
) -> Result<Vec<WorldSummary>, CommandError> {
    Ok(state.listening.worlds().await?)
}

/// One world with its setting and cast; `None` if it is gone.
#[tauri::command]
#[specta::specta]
pub async fn listening_world(
    state: State<'_, AppState>,
    name: String,
) -> Result<Option<WorldDetail>, CommandError> {
    Ok(state.listening.world(&name).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn listening_create_world(
    state: State<'_, AppState>,
    name: String,
    setting: String,
) -> Result<(), CommandError> {
    Ok(state.listening.create_world(&name, &setting).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn listening_update_setting(
    state: State<'_, AppState>,
    name: String,
    setting: String,
) -> Result<(), CommandError> {
    Ok(state.listening.update_setting(&name, &setting).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn listening_add_cast(
    state: State<'_, AppState>,
    world: String,
    character: String,
    role: Role,
) -> Result<(), CommandError> {
    Ok(state.listening.add_cast(&world, &character, &role).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn listening_set_role(
    state: State<'_, AppState>,
    world: String,
    character: String,
    role: Role,
) -> Result<(), CommandError> {
    Ok(state.listening.set_role(&world, &character, &role).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn listening_remove_cast(
    state: State<'_, AppState>,
    world: String,
    character: String,
) -> Result<bool, CommandError> {
    Ok(state.listening.remove_cast(&world, &character).await?)
}

/// Deletes the world with its episodes and their audio.
#[tauri::command]
#[specta::specta]
pub async fn listening_delete_world(
    state: State<'_, AppState>,
    name: String,
) -> Result<bool, CommandError> {
    Ok(state.listening.delete_world(&name).await?)
}
