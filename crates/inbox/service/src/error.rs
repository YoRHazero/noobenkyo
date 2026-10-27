use thiserror::Error;

/// Why the inbox could not be used, in Chinese.
#[derive(Debug, Error)]
pub enum InboxError {
    #[error("无法读写 inbox：{0}")]
    Io(#[from] std::io::Error),
    #[error("inbox 里的 {file} 读不出来，已经删掉：{reason}")]
    Unreadable { file: String, reason: String },
    #[error("{0} 不能当名字：不能为空，不能以点开头，也不能含有路径分隔符")]
    BadName(String),
    #[error("inbox 里已经有 {0}")]
    Taken(String),
    #[error("inbox 里没有 {0}")]
    Missing(String),
    #[error("inbox 里的 {0} 不是条目")]
    NotEntries(String),
    #[error("inbox 里的 {0} 不是剧本")]
    NotScript(String),
    /// The script is stored, so saving it again would store it twice.
    #[error("已经保存，但 inbox 里的 {name} 没能删掉：{reason}")]
    SavedButKept {
        name: String,
        reason: Box<InboxError>,
    },
    #[error("无法监听 inbox：{0}")]
    Watch(#[from] notify::Error),
}
