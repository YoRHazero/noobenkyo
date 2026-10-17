use serde::Serialize;

/// Progress of a running dictionary import.
#[derive(Clone, Debug, Serialize, specta::Type, tauri_specta::Event)]
pub struct ImportProgressEvent(pub dictionary::ImportProgress);
