use listening::ScriptPreview;

/// What a script would be stored as, for the script preview, or the problems
/// that stop it. Asynchronous, although pure, because its time grows with
/// the text.
#[tauri::command]
#[specta::specta]
pub async fn listening_preview_script(text: String) -> ScriptPreview {
    listening::preview_script(&text)
}
