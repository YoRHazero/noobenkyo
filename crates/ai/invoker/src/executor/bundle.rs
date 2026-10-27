use std::collections::BTreeMap;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use sha2::{Digest, Sha256};

/// The executor's Python package, built into the app, by path in it.
const FILES: [(&str, &str); 5] = [
    ("__main__.py", include_str!("../../executor/__main__.py")),
    ("protocol.py", include_str!("../../executor/protocol.py")),
    (
        "runtimes/__init__.py",
        include_str!("../../executor/runtimes/__init__.py"),
    ),
    (
        "runtimes/llama_cpp.py",
        include_str!("../../executor/runtimes/llama_cpp.py"),
    ),
    (
        "runtimes/qwen_tts.py",
        include_str!("../../executor/runtimes/qwen_tts.py"),
    ),
];

/// Run by `python -` on the GPU machine to unpack the package.
const INSTALL: &str = include_str!("../../install.py");

/// The executor's code as sent to a machine, named by its hash so each
/// version gets its own directory there.
pub(crate) struct Bundle {
    hash: String,
    install_script: String,
}

impl Bundle {
    pub(crate) fn new() -> Self {
        let files: BTreeMap<&str, &str> = FILES.into_iter().collect();
        let json = serde_json::to_string(&files).expect("file names and texts serialize");
        let hash: String = Sha256::digest(json.as_bytes())[..8]
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        // Base64 keeps the code clear of Python's string quoting.
        let install_script = format!(
            "{INSTALL}\ninstall(\"{hash}\", \"{}\")\n",
            BASE64.encode(json)
        );
        Self {
            hash,
            install_script,
        }
    }

    pub(crate) fn hash(&self) -> &str {
        &self.hash
    }

    pub(crate) fn install_script(&self) -> &str {
        &self.install_script
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_hash_names_this_version() {
        let bundle = Bundle::new();
        assert_eq!(bundle.hash().len(), 16);
        assert!(bundle.hash().chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(Bundle::new().hash(), bundle.hash());
    }

    #[test]
    fn the_script_ends_by_installing_every_file() {
        let bundle = Bundle::new();
        let call = bundle.install_script().lines().last().unwrap();
        let prefix = format!("install(\"{}\", \"", bundle.hash());
        let encoded = call
            .strip_prefix(&prefix)
            .unwrap()
            .strip_suffix("\")")
            .unwrap();
        let files: BTreeMap<String, String> =
            serde_json::from_slice(&BASE64.decode(encoded).unwrap()).unwrap();
        let names: Vec<&str> = files.keys().map(String::as_str).collect();
        assert_eq!(
            names,
            [
                "__main__.py",
                "protocol.py",
                "runtimes/__init__.py",
                "runtimes/llama_cpp.py",
                "runtimes/qwen_tts.py"
            ]
        );
    }
}
