//! Runs models on GPU machines for the app: starts an executor (a Python
//! process) over SSH when a call needs one, sends it the call, brings the
//! result back, and closes it when idle. Knows nothing of characters, worlds
//! or scripts, only single model calls. Apart from calls, it can probe a
//! machine for the model page. See dev_docs/Invoker.md.

mod catalog;
mod error;
mod executor;
mod invoker;
mod llm;
mod probe;
mod protocol;
mod schedule;
mod settings;
mod times;
mod tts;

pub use error::InvokeError;
pub use invoker::Invoker;
pub use llm::{LlmFinish, LlmProgress, LlmReply};
pub use probe::{FoundRuntime, MachineReport, probe};
pub use settings::{
    CommandLayer, InvokerSettings, InvokerSettingsError, LlamaCppModel, Machine, ProbeDirs,
    QwenTtsModel, Runtime, TtsOp,
};
pub use tts::{TtsCloneRequest, TtsDesignRequest, TtsLanguage, TtsReply};
