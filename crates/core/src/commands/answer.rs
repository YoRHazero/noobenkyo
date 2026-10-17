use study::{Rating, Zone};
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;
use crate::convert::sense_key;
use crate::flows::fill_zone::fill_zone;
use crate::flows::record_answer::record_answer;
use crate::flows::settle::settle_and_remove;

#[tauri::command]
#[specta::specta]
pub async fn rate_answer(
    state: State<'_, AppState>,
    zone: Zone,
    rating: Rating,
) -> Result<(), CommandError> {
    let mut study = state.study.lock().await;
    let outcome = study.rate(zone, rating)?;
    if let Some(record) = outcome.record {
        record_answer(&state.dictionary, record).await?;
    }
    if let Some(settlement) = outcome.settlement {
        settle_and_remove(&state.dictionary, &mut study, zone, settlement).await?;
    }
    fill_zone(&state.dictionary, &mut study, zone).await
}

#[tauri::command]
#[specta::specta]
pub async fn skip_sense(state: State<'_, AppState>, zone: Zone) -> Result<(), CommandError> {
    let mut study = state.study.lock().await;
    let settlement = study.skip(zone)?;
    settle_and_remove(&state.dictionary, &mut study, zone, settlement).await?;
    fill_zone(&state.dictionary, &mut study, zone).await
}

/// Deletes the shown sentence and lets the sense leave without settling.
#[tauri::command]
#[specta::specta]
pub async fn report_sentence(state: State<'_, AppState>, zone: Zone) -> Result<(), CommandError> {
    let mut study = state.study.lock().await;
    let reported = study.report_sentence(zone)?;
    let key = sense_key::to_dictionary(reported.key.clone());
    state
        .dictionary
        .remove_sentence(&key, &reported.marked)
        .await?;
    study.remove(zone, &reported.key)?;
    fill_zone(&state.dictionary, &mut study, zone).await
}
