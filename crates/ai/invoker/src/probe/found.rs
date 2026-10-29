//! What probe.py prints: the facts as found on the machine, before they are
//! read against the layout.

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub(crate) struct Found {
    pub models_dir: bool,
    /// Each path asked about, and whether it is a file.
    pub files: HashMap<String, bool>,
    /// The directories one level down whose config.json says qwen3_tts.
    pub tts: Vec<FoundTts>,
    pub gguf: Vec<FoundGguf>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub(crate) struct FoundTts {
    pub name: String,
    pub path: String,
    /// config.json's tts_model_type: base, voice_design, custom_voice.
    #[serde(rename = "type")]
    pub kind: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub(crate) struct FoundGguf {
    pub name: String,
    pub path: String,
}

impl Found {
    pub(crate) fn has_file(&self, path: &str) -> bool {
        self.files.get(path).copied().unwrap_or(false)
    }
}

/// The report in the probe's output: its last line that reads as one. A
/// login banner or a warning may come first.
pub(crate) fn parse(stdout: &str) -> Option<Found> {
    stdout
        .lines()
        .rev()
        .find_map(|line| serde_json::from_str(line.trim()).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_last_line_that_is_a_report() {
        let stdout = "Welcome to Ubuntu\n\
            {\"models_dir\": true, \"files\": {\"/py\": false}, \
             \"tts\": [{\"name\": \"Base\", \"path\": \"/m/Base\", \"type\": \"base\"}], \
             \"gguf\": [{\"name\": \"a-Q8_0\", \"path\": \"/m/a/a-Q8_0.gguf\"}]}\n";
        let found = parse(stdout).unwrap();
        assert!(found.models_dir);
        assert!(!found.has_file("/py"));
        assert!(!found.has_file("/not/asked"));
        assert_eq!(found.tts[0].kind.as_deref(), Some("base"));
        assert_eq!(found.gguf[0].name, "a-Q8_0");
        assert_eq!(parse("python3: not found\n"), None);
    }
}
