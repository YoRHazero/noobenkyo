use study::{ReviewMode, Zone, ZoneStatus};
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;
use crate::flows::fill_zone::fill_zone;
use crate::flows::settle::write_back;
use crate::flows::start_group::start_group;

#[tauri::command]
#[specta::specta]
pub async fn zone_status(
    state: State<'_, AppState>,
    zone: Zone,
) -> Result<ZoneStatus, CommandError> {
    let mut study = state.study.lock().await;
    fill_zone(&state.dictionary, &mut study, zone).await?;
    Ok(study.status(zone))
}

/// Settles every review sense, then switches the mode, which empties the review zone.
#[tauri::command]
#[specta::specta]
pub async fn set_review_mode(
    state: State<'_, AppState>,
    mode: ReviewMode,
) -> Result<ZoneStatus, CommandError> {
    let mut study = state.study.lock().await;
    if study.review_mode() != mode {
        for settlement in study.review_settlements() {
            write_back(&state.dictionary, &settlement).await?;
        }
        study.set_review_mode(mode)?;
    }
    fill_zone(&state.dictionary, &mut study, Zone::Review).await?;
    Ok(study.status(Zone::Review))
}

#[tauri::command]
#[specta::specta]
pub async fn start_review_group(state: State<'_, AppState>) -> Result<ZoneStatus, CommandError> {
    let mut study = state.study.lock().await;
    start_group(&state.dictionary, &mut study).await?;
    Ok(study.status(Zone::Review))
}
