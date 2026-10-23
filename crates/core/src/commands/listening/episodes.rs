use listening::{EpisodeDetail, EpisodeSummary, Uuid};
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

#[tauri::command]
#[specta::specta]
pub async fn listening_episodes(
    state: State<'_, AppState>,
    world: String,
) -> Result<Vec<EpisodeSummary>, CommandError> {
    Ok(state.listening.episodes(&world).await?)
}

/// One episode with its lines; `None` if it is gone.
#[tauri::command]
#[specta::specta]
pub async fn listening_episode(
    state: State<'_, AppState>,
    episode: Uuid,
) -> Result<Option<EpisodeDetail>, CommandError> {
    Ok(state.listening.episode(episode).await?)
}

/// Replaces the number, title and summary together. Returns the problems
/// that rejected them, empty when they were replaced.
#[tauri::command]
#[specta::specta]
pub async fn listening_update_episode(
    state: State<'_, AppState>,
    episode: Uuid,
    number: f64,
    title: String,
    summary: String,
) -> Result<Vec<String>, CommandError> {
    Ok(state
        .listening
        .update_episode(episode, number, &title, &summary)
        .await?)
}

#[tauri::command]
#[specta::specta]
pub async fn listening_delete_episode(
    state: State<'_, AppState>,
    episode: Uuid,
) -> Result<bool, CommandError> {
    Ok(state.listening.delete_episode(episode).await?)
}
