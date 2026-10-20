use study::Next;
use tauri::State;

use super::current::current;
use crate::app_state::AppState;
use crate::command_error::CommandError;
use crate::payloads::QuizPayload;
use crate::steps::content::content_of;

/// The quiz in progress, or else a new one; `None` when the pool and the
/// queue are both empty. A sense whose content is gone leaves the session,
/// and the next one is asked instead.
#[tauri::command]
#[specta::specta]
pub async fn review_next(state: State<'_, AppState>) -> Result<Option<QuizPayload>, CommandError> {
    let mut study = state.study.lock().await;
    loop {
        match current(&mut study)?.next() {
            Next::Pending(quiz) => return QuizPayload::render(quiz).map(Some),
            Next::NeedContent(key) => {
                let content = content_of(&state.dictionary, &key).await?;
                if let Some(quiz) = current(&mut study)?.make_quiz(content)? {
                    return QuizPayload::render(quiz).map(Some);
                }
            }
            Next::Empty => return Ok(None),
        }
    }
}
