use dictionary_word::rules::sentence::SentenceError;
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DictionaryError {
    #[error("数据库错误：{0}")]
    Db(#[from] DbErr),
    #[error("无法创建数据目录：{0}")]
    Io(#[from] std::io::Error),
    #[error("例句格式错误：{0}")]
    Sentence(#[from] SentenceError),
}
