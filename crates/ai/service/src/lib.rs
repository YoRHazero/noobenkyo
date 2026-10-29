//! The ai group's public face: `AiService`, its settings, and the
//! sub-crate types that leave the group. The app's own use of models lives
//! here; it depends on no other group, so core hands it the data in these
//! types and stores what comes back. See dev_docs/Pipeline.md.

mod error;
mod probe;
mod service;
mod settings;
mod settings_file;
mod voice;

pub use error::AiError;
pub use probe::MachineProbe;
pub use service::AiService;
pub use settings::{AiSettings, ModelChoice};

pub use ai_invoker::{
    CommandLayer, FoundRuntime, InvokerSettings, InvokerSettingsError, LlamaCppModel, Machine,
    MachineReport, ProbeDirs, QwenTtsModel, Runtime, TtsOp,
};
pub use ai_pipeline::{Language, PipelineError, SpeakItem, Spoken, VoiceSample};
