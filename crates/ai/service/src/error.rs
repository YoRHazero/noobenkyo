use ai_tools::InboxError;
use listening::ListeningError;
use thiserror::Error;

/// Why the inbox could not be used, in Chinese.
#[derive(Debug, Error)]
pub enum AiError {
    #[error(transparent)]
    Inbox(#[from] InboxError),
    #[error("inbox 里的 {0} 不是条目")]
    NotEntries(String),
    #[error("inbox 里的 {0} 不是剧本")]
    NotScript(String),
    #[error(transparent)]
    Listening(#[from] ListeningError),
    /// The script is stored, so saving it again would store it twice.
    #[error("已经保存，但 inbox 里的 {name} 没能删掉：{reason}")]
    SavedButKept { name: String, reason: InboxError },
    #[error("无法监听 inbox：{0}")]
    Watch(#[from] notify::Error),
}
