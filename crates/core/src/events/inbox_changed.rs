use agent::{AgentError, AgentService, InboxWatcher};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_specta::Event;

/// Something in the inbox appeared, was replaced or was removed. It does not
/// say what or by whom: the frontend reads the inbox again.
#[derive(Clone, Debug, Serialize, Deserialize, specta::Type, Event)]
pub struct InboxChanged;

/// Watches the inbox and passes every change on to the frontend, for as
/// long as the returned watcher is kept.
pub fn forward(app: &AppHandle, agent: &AgentService) -> Result<InboxWatcher, AgentError> {
    let app = app.clone();
    agent.watch(move || {
        // Fails only when the app is shutting down.
        let _ = InboxChanged.emit(&app);
    })
}
