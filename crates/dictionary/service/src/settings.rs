use serde::{Deserialize, Serialize};

/// Dictionary settings.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename = "DictionarySettings")]
pub struct Settings {
    /// Whether answers are logged. Off by default.
    pub log_enabled: bool,
}
