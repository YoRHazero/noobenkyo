//! What the invoker is told about the GPU machines: how to reach each one and
//! which runtimes and models it has. The values live in ai's settings; the
//! caller hands them over.

mod check;
mod command_layer;

use serde::{Deserialize, Serialize};
use specta_typescript::Number;

pub use check::InvokerSettingsError;
pub(crate) use check::{check_machine, is_safe_on_command_line};
pub use command_layer::CommandLayer;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct InvokerSettings {
    /// In order of preference. Empty means there is no backend.
    pub machines: Vec<Machine>,
    /// How long an executor may sit unused before it is closed.
    #[specta(type = Number)]
    pub idle_timeout_secs: u64,
    /// How long ssh may take to connect before the next machine is tried.
    #[specta(type = Number)]
    pub connect_timeout_secs: u64,
}

impl Default for InvokerSettings {
    fn default() -> Self {
        Self {
            machines: Vec::new(),
            idle_timeout_secs: 600,
            connect_timeout_secs: 5,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct Machine {
    /// For display and logs.
    pub name: String,
    /// What ssh takes as the destination: an alias from ~/.ssh/config or
    /// user@host. None runs the executor on this computer.
    pub ssh: Option<String>,
    /// Commands wrapped around the executor once on the machine, outermost
    /// first, such as `wsl -e`. Each is one string in JSON.
    #[specta(type = Vec<String>)]
    pub command_layers: Vec<CommandLayer>,
    pub runtimes: Vec<Runtime>,
    /// Where probing looks; calls never read it. None until it is set.
    #[serde(default)]
    pub probe: Option<ProbeDirs>,
}

/// The directories a machine is probed in: what the model page needs to
/// find its runtimes and models.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct ProbeDirs {
    /// Where the models are: qwen-tts ones a level down, GGUF files anywhere below.
    pub models: String,
    /// Where qwen-tts is installed; its python is `.venv/bin/python` in it.
    pub qwen_tts: Option<String>,
    /// Where llama.cpp is installed; its server is `bin/llama-server` in it.
    pub llama_cpp: Option<String>,
    /// The python llama.cpp's executor runs on when there is no qwen-tts;
    /// `/usr/bin/python3` if left out.
    pub llama_python: Option<String>,
}

/// An environment on a machine and the models it runs. A model's runtime
/// decides which python starts its executor and which calls it can take.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(tag = "kind")]
pub enum Runtime {
    LlamaCpp {
        python: String,
        server: String,
        models: Vec<LlamaCppModel>,
    },
    QwenTts {
        python: String,
        models: Vec<QwenTtsModel>,
    },
}

impl Runtime {
    pub fn python(&self) -> &str {
        match self {
            Runtime::LlamaCpp { python, .. } | Runtime::QwenTts { python, .. } => python,
        }
    }
}

/// A GGUF model for llama-server; it only takes `llm_chat`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct LlamaCppModel {
    pub name: String,
    pub path: String,
    /// llama-server's arguments that depend on the machine, such as `-c` and `-ngl`.
    pub args: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct QwenTtsModel {
    pub name: String,
    pub op: TtsOp,
    pub path: String,
}

/// Which TTS call a qwen-tts model takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum TtsOp {
    TtsClone,
    TtsDesign,
}
