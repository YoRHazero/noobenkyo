//! settings.json in the study data directory.

use std::path::Path;

use serde_json::Value;

use crate::Settings;

const FILE_NAME: &str = "settings.json";

/// Reads the settings. Fields missing from the file take their defaults;
/// an unreadable or invalid file gives the defaults.
pub fn load(dir: &Path) -> Settings {
    std::fs::read_to_string(dir.join(FILE_NAME))
        .ok()
        .and_then(|text| serde_json::from_str::<Value>(&text).ok())
        .and_then(|file| {
            let defaults = serde_json::to_value(Settings::default()).ok()?;
            serde_json::from_value::<Settings>(overlay(defaults, file)).ok()
        })
        .filter(|settings| settings.validate().is_ok())
        .unwrap_or_default()
}

/// Puts the file's values over the defaults, object by object.
fn overlay(defaults: Value, file: Value) -> Value {
    match (defaults, file) {
        (Value::Object(mut base), Value::Object(top)) => {
            for (key, value) in top {
                let merged = match base.remove(&key) {
                    Some(default) => overlay(default, value),
                    None => value,
                };
                base.insert(key, merged);
            }
            Value::Object(base)
        }
        (_, file) => file,
    }
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
    fn missing_fields_take_defaults() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join(FILE_NAME),
            r#"{"workspace": {"review_capacity": 5}}"#,
        )
        .unwrap();
        let settings = load(dir.path());
        assert_eq!(settings.workspace.review_capacity, 5);
        assert_eq!(settings.workspace.learning_capacity, 10);
        assert_eq!(settings.quiz, Default::default());
    }
}
