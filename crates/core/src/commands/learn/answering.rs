//! Answering the quiz in progress, skipping its sense, or reporting its
//! sentence. None of them hands back the next quiz: that is `learn_next`.

use study::Rating;
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;
use crate::payloads::RevealPayload;
use crate::steps::bad_sentence::delete_bad_sentence;
use crate::steps::write_back::write_back;

#[tauri::command]
#[specta::specta]
pub async fn learn_submit_answer(
    state: State<'_, AppState>,
    input: String,
) -> Result<RevealPayload, CommandError> {
    let mut study = state.study.lock().await;
    let reference = study.learn().submit_answer(input)?;
    RevealPayload::render(reference)
}

#[tauri::command]
#[specta::specta]
pub async fn learn_confirm_answer(
    state: State<'_, AppState>,
    rating: Rating,
) -> Result<(), CommandError> {
    let mut study = state.study.lock().await;
    if let Some(write) = study.learn().confirm_answer(rating)? {
        write_back(&state.dictionary, write).await?;
    }
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn learn_skip(state: State<'_, AppState>) -> Result<(), CommandError> {
    let mut study = state.study.lock().await;
    let write = study.learn().skip()?;
    write_back(&state.dictionary, write).await
}

#[tauri::command]
#[specta::specta]
pub async fn learn_report_sentence(state: State<'_, AppState>) -> Result<(), CommandError> {
    let mut study = state.study.lock().await;
    let bad = study.learn().report_sentence()?;
    delete_bad_sentence(&state.dictionary, bad).await
}
