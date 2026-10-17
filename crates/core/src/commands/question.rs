use study::Zone;
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;
use crate::flows::question;
use crate::flows::render::render;
use crate::payloads::{QuestionPayload, RevealPayload};

/// The zone's current question; the same one until it is answered. `None` when the zone is empty.
#[tauri::command]
#[specta::specta]
pub async fn current_question(
    state: State<'_, AppState>,
    zone: Zone,
) -> Result<Option<QuestionPayload>, CommandError> {
    let mut study = state.study.lock().await;
    question::current_question(&state.dictionary, &mut study, zone).await
}

#[tauri::command]
#[specta::specta]
pub async fn submit_answer(
    state: State<'_, AppState>,
    zone: Zone,
    input: String,
) -> Result<RevealPayload, CommandError> {
    let reveal = state.study.lock().await.submit(zone, input)?;
    Ok(RevealPayload {
        view: reveal.view,
        fragments: render(reveal.sentence)?,
    })
}
