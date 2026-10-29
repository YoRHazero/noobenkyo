//! Helpers for the behaviour tests.

use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use ai::{AiSettings, InvokerSettings, Machine, ModelChoice, QwenTtsModel, Runtime, TtsOp};

/// Settings with one machine on this computer whose "python" never answers,
/// so a call that reaches it waits until it is dropped. The script lives in
/// `dir`.
pub fn hanging_machine(dir: &Path) -> AiSettings {
    let python = dir.join("python");
    std::fs::write(&python, "#!/bin/sh\nexec sleep 30\n").unwrap();
    std::fs::set_permissions(&python, std::fs::Permissions::from_mode(0o755)).unwrap();
    AiSettings {
        invoker: InvokerSettings {
            machines: vec![Machine {
                name: "here".into(),
                ssh: None,
                command_layers: vec![],
                runtimes: vec![Runtime::QwenTts {
                    python: python.to_str().unwrap().into(),
                    models: vec![QwenTtsModel {
                        name: "qwen3-tts-1.7b-voicedesign".into(),
                        op: TtsOp::TtsDesign,
                        path: "/nowhere/voicedesign".into(),
                    }],
                }],
                probe: None,
            }],
            ..InvokerSettings::default()
        },
        models: ModelChoice {
            speak: None,
            design_voice: Some("qwen3-tts-1.7b-voicedesign".into()),
        },
    }
}

pub fn write_settings_file(dir: &Path, text: &str) {
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(dir.join("settings.json"), text).unwrap();
}
