use study_quiz::AnswerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LearnError {
    #[error("读写学习区失败：{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Answer(#[from] AnswerError),
}
