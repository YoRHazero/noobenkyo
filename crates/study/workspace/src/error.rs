use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceError {
    #[error("当前没有题目")]
    NoQuestion,
    #[error("当前题目没有例句")]
    NoSentence,
}
