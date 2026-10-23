use listening::{Language, LineId};
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;

/// Replaces one language of a line and drops its audio in that language.
/// Returns the problems that rejected the text, empty when it was replaced
/// or unchanged.
#[tauri::command]
#[specta::specta]
pub async fn listening_update_line(
    state: State<'_, AppState>,
    line: LineId,
    language: Language,
    text: String,
) -> Result<Vec<String>, CommandError> {
    Ok(state.listening.update_line(line, language, &text).await?)
}

/// A line's audio in `language`, to play; `None` if it has none.
#[tauri::command]
#[specta::specta]
pub async fn listening_audio(
    state: State<'_, AppState>,
    line: LineId,
    language: Language,
) -> Result<Option<Vec<u8>>, CommandError> {
    Ok(state.listening.audio(line, language).await?)
}

/// Drops the lines' audio in `language`. Returns how many recordings were
/// dropped.
#[tauri::command]
#[specta::specta]
pub async fn listening_clear_audio(
    state: State<'_, AppState>,
    lines: Vec<LineId>,
    language: Language,
) -> Result<u32, CommandError> {
    Ok(state.listening.clear_audio(&lines, language).await?)
}

/// Returns how many lines were deleted.
#[tauri::command]
#[specta::specta]
pub async fn listening_delete_lines(
    state: State<'_, AppState>,
    lines: Vec<LineId>,
) -> Result<u32, CommandError> {
    Ok(state.listening.delete_lines(&lines).await?)
}
