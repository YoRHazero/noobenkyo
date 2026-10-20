use study_learn::LearnError;
use study_review::SessionOptionsError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StudyError {
    #[error("读写文件失败：{0}")]
    Io(#[from] std::io::Error),
    #[error("设置不合法：{0}")]
    InvalidSettings(String),
    #[error("复习选项不合法：{0}")]
    InvalidOptions(#[from] SessionOptionsError),
    #[error("{0}")]
    Learn(#[from] LearnError),
}
