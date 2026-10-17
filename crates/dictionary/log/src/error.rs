use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LogError {
    #[error("读写文件失败：{0}")]
    Io(#[from] std::io::Error),
    #[error("数据库错误：{0}")]
    Db(#[from] DbErr),
}
