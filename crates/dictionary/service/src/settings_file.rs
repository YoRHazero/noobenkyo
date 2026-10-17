//! settings.json in the dictionary's data directory.

use std::path::Path;

use crate::Settings;

const FILE_NAME: &str = "settings.json";

/// Reads the settings; a missing or unreadable file gives the defaults.
pub fn load(dir: &Path) -> Settings {
    std::fs::read_to_string(dir.join(FILE_NAME))
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

/// Writes the settings through a temporary file, then renames it into place.
pub fn save(dir: &Path, settings: &Settings) -> std::io::Result<()> {
    let text = serde_json::to_string_pretty(settings).map_err(std::io::Error::other)?;
    let temp = dir.join(format!("{FILE_NAME}.tmp"));
    std::fs::write(&temp, text)?;
    std::fs::rename(temp, dir.join(FILE_NAME))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saves_and_loads() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(load(dir.path()), Settings::default());
        save(dir.path(), &Settings { log_enabled: true }).unwrap();
        assert_eq!(load(dir.path()), Settings { log_enabled: true });
    }

    #[test]
    fn missing_fields_take_defaults() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(FILE_NAME), "{}").unwrap();
        assert_eq!(load(dir.path()), Settings::default());
    }
}
