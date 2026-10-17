use study_workspace::WorkspaceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StudyError {
    #[error("读写文件失败：{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Workspace(#[from] WorkspaceError),
    #[error("设置不合法：{0}")]
    InvalidSettings(String),
}
