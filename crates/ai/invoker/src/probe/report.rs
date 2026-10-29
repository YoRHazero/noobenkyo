//! What a probe tells the model page, read from what was found against the
//! layout: each runtime as it would go into the settings.

use serde::Serialize;

use super::found::Found;
use super::paths;
use crate::{LlamaCppModel, ProbeDirs, QwenTtsModel, Runtime, TtsOp};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MachineReport {
    /// Could not be reached, or nothing on it could run the probe.
    Offline { reason: String },
    Online {
        /// Whether the models folder is there; `None` when none is given.
        models_dir: Option<bool>,
        /// Only when a qwen-tts directory is given.
        qwen_tts: Option<FoundRuntime>,
        /// Only when a llama.cpp directory is given.
        llama_cpp: Option<FoundRuntime>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct FoundRuntime {
    /// Its python and server by the layout, and every model found for it.
    /// The settings take it with only the models the user ticks.
    pub runtime: Runtime,
    /// Its python or llama-server is not where the layout puts it.
    pub problem: Option<String>,
}

/// llama-server's arguments for a model found by probing; not open to
/// change yet (user's decision). The threads are left to llama-server.
const LLAMA_ARGS: [&str; 10] = [
    "-c",
    "16384",
    "-ngl",
    "auto",
    "--fit",
    "on",
    "--fit-target",
    "512",
    "--reasoning",
    "off",
];

/// The files the probe is asked about: the ones `online` reads.
pub(crate) fn files_to_check(dirs: &ProbeDirs) -> Vec<String> {
    let mut files = Vec::new();
    if let Some(dir) = &dirs.qwen_tts {
        files.push(paths::qwen_tts_python(dir));
    }
    if let Some(dir) = &dirs.llama_cpp {
        files.push(paths::llama_python(dirs));
        files.push(paths::llama_server(dir));
    }
    files.dedup();
    files
}

pub(crate) fn online(dirs: &ProbeDirs, found: &Found) -> MachineReport {
    let missing = |paths: &[&str]| {
        let gone: Vec<&str> = paths
            .iter()
            .copied()
            .filter(|path| !found.has_file(path))
            .collect();
        (!gone.is_empty()).then(|| format!("找不到 {}", gone.join("、")))
    };
    let qwen_tts = dirs.qwen_tts.as_deref().map(|dir| {
        let python = paths::qwen_tts_python(dir);
        let models = found
            .tts
            .iter()
            .filter_map(|model| {
                let op = match model.kind.as_deref() {
                    Some("base") => TtsOp::TtsClone,
                    Some("voice_design") => TtsOp::TtsDesign,
                    // custom_voice: no call takes it yet.
                    _ => return None,
                };
                Some(QwenTtsModel {
                    name: model.name.clone(),
                    op,
                    path: model.path.clone(),
                })
            })
            .collect();
        FoundRuntime {
            problem: missing(&[&python]),
            runtime: Runtime::QwenTts { python, models },
        }
    });
    let llama_cpp = dirs.llama_cpp.as_deref().map(|dir| {
        let python = paths::llama_python(dirs);
        let server = paths::llama_server(dir);
        let models = found
            .gguf
            .iter()
            .map(|model| LlamaCppModel {
                name: model.name.clone(),
                path: model.path.clone(),
                args: LLAMA_ARGS.map(str::to_owned).to_vec(),
            })
            .collect();
        FoundRuntime {
            problem: missing(&[&python, &server]),
            runtime: Runtime::LlamaCpp {
                python,
                server,
                models,
            },
        }
    });
    MachineReport::Online {
        models_dir: (!dirs.models.trim().is_empty()).then_some(found.models_dir),
        qwen_tts,
        llama_cpp,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::probe::found::{FoundGguf, FoundTts};

    fn dirs() -> ProbeDirs {
        ProbeDirs {
            models: "/m".into(),
            qwen_tts: Some("/opt/qwen-tts".into()),
            llama_cpp: Some("/opt/llama.cpp".into()),
            llama_python: None,
        }
    }

    fn tts(name: &str, kind: &str) -> FoundTts {
        FoundTts {
            name: name.into(),
            path: format!("/m/{name}"),
            kind: Some(kind.into()),
        }
    }

    #[test]
    fn asks_about_each_file_once() {
        assert_eq!(
            files_to_check(&dirs()),
            [
                "/opt/qwen-tts/.venv/bin/python",
                "/opt/llama.cpp/bin/llama-server"
            ]
        );
        let neither = ProbeDirs {
            models: "/m".into(),
            ..ProbeDirs::default()
        };
        assert!(files_to_check(&neither).is_empty());
    }

    #[test]
    fn reads_what_was_found_into_runtimes() {
        let found = Found {
            models_dir: true,
            files: HashMap::from([
                ("/opt/qwen-tts/.venv/bin/python".to_owned(), true),
                ("/opt/llama.cpp/bin/llama-server".to_owned(), false),
            ]),
            tts: vec![
                tts("Base", "base"),
                tts("VoiceDesign", "voice_design"),
                tts("CustomVoice", "custom_voice"),
            ],
            gguf: vec![FoundGguf {
                name: "a-Q8_0".into(),
                path: "/m/a/a-Q8_0.gguf".into(),
            }],
        };
        let MachineReport::Online {
            models_dir,
            qwen_tts: Some(qwen_tts),
            llama_cpp: Some(llama_cpp),
        } = online(&dirs(), &found)
        else {
            panic!("both runtimes are given");
        };
        assert_eq!(models_dir, Some(true));
        assert_eq!(qwen_tts.problem, None);
        let Runtime::QwenTts { python, models } = qwen_tts.runtime else {
            panic!("qwen-tts");
        };
        assert_eq!(python, "/opt/qwen-tts/.venv/bin/python");
        let ops: Vec<(&str, TtsOp)> = models.iter().map(|m| (m.name.as_str(), m.op)).collect();
        assert_eq!(
            ops,
            [("Base", TtsOp::TtsClone), ("VoiceDesign", TtsOp::TtsDesign)]
        );
        assert_eq!(
            llama_cpp.problem.as_deref(),
            Some("找不到 /opt/llama.cpp/bin/llama-server")
        );
        let Runtime::LlamaCpp { python, models, .. } = llama_cpp.runtime else {
            panic!("llama.cpp");
        };
        assert_eq!(python, "/opt/qwen-tts/.venv/bin/python");
        assert_eq!(models[0].path, "/m/a/a-Q8_0.gguf");
        assert_eq!(models[0].args[..2], ["-c", "16384"]);
    }

    #[test]
    fn leaves_out_the_runtimes_not_given() {
        let only_models = ProbeDirs {
            models: "/m".into(),
            ..ProbeDirs::default()
        };
        let found = Found {
            models_dir: false,
            files: HashMap::new(),
            tts: vec![],
            gguf: vec![],
        };
        assert_eq!(
            online(&only_models, &found),
            MachineReport::Online {
                models_dir: Some(false),
                qwen_tts: None,
                llama_cpp: None,
            }
        );
        // No folder given: whether it is there is not a question.
        assert_eq!(
            online(&ProbeDirs::default(), &found),
            MachineReport::Online {
                models_dir: None,
                qwen_tts: None,
                llama_cpp: None,
            }
        );
    }
}
