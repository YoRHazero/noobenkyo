use listening::Language;
use tauri::State;

use crate::app_state::AppState;
use crate::command_error::CommandError;
use crate::convert::language;

/// Makes up a voice from `prompt` saying `text` and returns the WAV, to be
/// listened to before it is stored as a reference. The seed is the
/// frontend's: one for both languages of a new character.
#[tauri::command]
#[specta::specta]
pub async fn ai_design_voice(
    state: State<'_, AppState>,
    prompt: String,
    language: Language,
    text: String,
    seed: u32,
) -> Result<Vec<u8>, CommandError> {
    let language = language::to_ai(language);
    Ok(state
        .ai
        .design_voice(&prompt, language, &text, u64::from(seed))
        .await?)
}
