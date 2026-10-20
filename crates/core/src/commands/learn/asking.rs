use study::Next;
use tauri::State;

use super::filling::fill_zone;
use crate::app_state::AppState;
use crate::command_error::CommandError;
use crate::payloads::QuizPayload;
use crate::steps::content::content_of;

/// The quiz in progress, or else a new one; `None` when there is nothing left
/// to learn. A sense whose content is gone leaves the zone, and the next one
/// is asked instead.
#[tauri::command]
#[specta::specta]
pub async fn learn_next(state: State<'_, AppState>) -> Result<Option<QuizPayload>, CommandError> {
    let mut study = state.study.lock().await;
    loop {
        fill_zone(&state.dictionary, &mut study).await?;
        match study.learn().next() {
            Next::Pending(quiz) => return QuizPayload::render(quiz).map(Some),
            Next::NeedContent(key) => {
                let content = content_of(&state.dictionary, &key).await?;
                if let Some(quiz) = study.learn().make_quiz(content)? {
                    return QuizPayload::render(quiz).map(Some);
                }
            }
            Next::Empty => return Ok(None),
        }
    }
}
