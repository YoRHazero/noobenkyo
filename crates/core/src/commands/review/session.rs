//! Starting a session when the review page opens one, ending it when the
//! page is left.

use dictionary::{Completeness, ContentFilter, ProgressState, SenseOrder};
use study::SessionOptions;
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;
use crate::convert::candidate;

/// Starts a session over every touched sense with sentences, replacing any
/// session there is; study leaves the mastered ones out.
#[tauri::command]
#[specta::specta]
pub async fn review_start(
    state: State<'_, AppState>,
    options: SessionOptions,
) -> Result<(), CommandError> {
    let mut study = state.study.lock().await;
    let filter = ContentFilter {
        text: None,
        completeness: vec![Completeness::WithSentence],
        progress: ProgressState::Touched,
    };
    let candidates = state
        .dictionary
        .filter_senses(&filter, &[], SenseOrder::Creation, None)
        .await?
        .into_iter()
        .map(candidate::to_study)
        .collect();
    Ok(study.start_review(candidates, options)?)
}

/// Ends the session; whatever was not written back is dropped.
#[tauri::command]
#[specta::specta]
pub async fn review_end(state: State<'_, AppState>) -> Result<(), CommandError> {
    state.study.lock().await.end_review();
    Ok(())
}
