use dictionary::{Fragment, SentenceDisplay};

use crate::command_error::CommandError;

#[tauri::command]
#[specta::specta]
pub fn dictionary_render_sentence(
    marked: String,
    display: SentenceDisplay,
) -> Result<Vec<Fragment>, CommandError> {
    Ok(dictionary::render_sentence(&marked, display)?)
}
