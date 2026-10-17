use dictionary_word::rules::marked::MarkedError;
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DictionaryError {
    #[error("数据库错误：{0}")]
    Db(#[from] DbErr),
    #[error("{0}")]
    Import(#[from] dictionary_import::ImportError),
    #[error("{0}")]
    Export(#[from] dictionary_export::ExportError),
    #[error("{0}")]
    Log(#[from] dictionary_log::LogError),
    #[error("读写文件失败：{0}")]
    Io(#[from] std::io::Error),
    #[error("例句格式错误：{0}")]
    Marked(#[from] MarkedError),
}
