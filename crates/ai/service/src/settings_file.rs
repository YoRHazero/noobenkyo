//! settings.json in the ai data directory. It is edited by hand for now, so
//! a file that cannot be used says why instead of falling back silently.

use std::path::Path;

use serde_json::Value;

use crate::AiSettings;

const FILE_NAME: &str = "settings.json";

/// Reads the settings. No file gives the defaults (no machines); fields
/// missing from the file take their defaults. A file that cannot be read,
/// parsed or validated gives the reason.
pub fn load(dir: &Path) -> Result<AiSettings, String> {
    let path = dir.join(FILE_NAME);
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(AiSettings::default());
        }
        Err(error) => return Err(format!("读不出：{error}")),
    };
    let file: Value = serde_json::from_str(&text).map_err(|error| error.to_string())?;
    let defaults =
        serde_json::to_value(AiSettings::default()).map_err(|error| error.to_string())?;
    let settings: AiSettings =
        serde_json::from_value(overlay(defaults, file)).map_err(|error| error.to_string())?;
    settings.validate().map_err(|error| error.to_string())?;
    Ok(settings)
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

/// Writes the settings to a temporary file, then renames it into place.
pub fn save(dir: &Path, settings: &AiSettings) -> std::io::Result<()> {
    let text = serde_json::to_string_pretty(settings).map_err(std::io::Error::other)?;
    let temp = dir.join(format!("{FILE_NAME}.tmp"));
    std::fs::write(&temp, text)?;
    std::fs::rename(temp, dir.join(FILE_NAME))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn the_file_wins_and_the_defaults_fill_in() {
        let defaults = json!({ "invoker": { "machines": [], "idle_timeout_secs": 600 } });
        let file = json!({ "invoker": { "idle_timeout_secs": 60 } });
        assert_eq!(
            overlay(defaults, file),
            json!({ "invoker": { "machines": [], "idle_timeout_secs": 60 } })
        );
    }
}
