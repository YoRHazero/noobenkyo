use listening_drama::DramaError;
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ListeningError {
    #[error("数据库错误：{0}")]
    Db(#[from] DbErr),
    #[error("无法创建数据目录：{0}")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Drama(#[from] DramaError),
    #[error("剧本不通过：{}", .0.join("；"))]
    Script(Vec<String>),
}
