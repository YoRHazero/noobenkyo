use ai_invoker::InvokerSettingsError;
use ai_pipeline::PipelineError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AiError {
    #[error("无法读写 ai 的数据目录：{0}")]
    Io(#[from] std::io::Error),
    /// The file found at startup could not be used; the app runs without
    /// machines until the settings are fixed.
    #[error("ai/settings.json 没有用上：{0}")]
    BadSettingsFile(String),
    #[error("{0}")]
    InvalidSettings(#[from] InvokerSettingsError),
    /// A model call is under way or waiting for a machine.
    #[error("还有模型调用在做或在等，现在改不了设置")]
    Busy,
    #[error(transparent)]
    Pipeline(#[from] PipelineError),
}
