//! Runs models on GPU machines for the app: starts an executor (a Python
//! process) over SSH when a call needs one, sends it the call, brings the
//! result back, and closes it when idle. Knows nothing of characters, worlds
//! or scripts, only single model calls. See dev_docs/Invoker.md.

mod catalog;
mod error;
mod executor;
mod invoker;
mod llm;
mod protocol;
mod schedule;
mod settings;
mod times;
mod tts;

pub use error::InvokeError;
pub use invoker::Invoker;
pub use llm::{LlmFinish, LlmProgress, LlmReply};
pub use settings::{
    CommandLayer, InvokerSettings, InvokerSettingsError, LlamaCppModel, Machine, QwenTtsModel,
    Runtime, TtsOp,
};
pub use tts::{TtsCloneRequest, TtsDesignRequest, TtsLanguage, TtsReply};
