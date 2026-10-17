use serde::{Deserialize, Serialize};

/// Dictionary settings. Missing fields in the file take their defaults.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(default, rename = "DictionarySettings")]
pub struct Settings {
    /// Whether answers are logged. Off by default.
    pub log_enabled: bool,
}
