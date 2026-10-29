//! The probe's script: probe.py with a call appended, carrying what to look
//! at. Base64 keeps the paths clear of Python's string quoting, so they may
//! have spaces.

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde::Serialize;

const PROBE: &str = include_str!("../../probe.py");

#[derive(Serialize)]
struct Look<'a> {
    models: &'a str,
    files: &'a [String],
}

/// Looks in `models` and checks whether each of `files` is there.
pub(crate) fn script(models: &str, files: &[String]) -> String {
    let look = serde_json::to_string(&Look { models, files }).expect("paths serialize");
    format!("{PROBE}\nprobe(\"{}\")\n", BASE64.encode(look))
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;

    #[test]
    fn ends_by_calling_probe_with_what_to_look_at() {
        let files = vec!["/opt/qwen tts/.venv/bin/python".to_owned()];
        let script = script("/My Models", &files);
        let call = script.lines().last().unwrap();
        let encoded = call
            .strip_prefix("probe(\"")
            .unwrap()
            .strip_suffix("\")")
            .unwrap();
        let look: Value = serde_json::from_slice(&BASE64.decode(encoded).unwrap()).unwrap();
        assert_eq!(
            look,
            json!({ "models": "/My Models", "files": ["/opt/qwen tts/.venv/bin/python"] })
        );
    }
}
