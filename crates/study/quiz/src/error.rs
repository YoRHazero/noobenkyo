use thiserror::Error;

/// Calls made out of order.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AnswerError {
    #[error("还没有选出下一题")]
    NothingChosen,
    #[error("当前没有题目")]
    NoQuiz,
    #[error("还没有提交答案")]
    NotSubmitted,
}
