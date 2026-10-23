use sea_orm::DbErr;
use thiserror::Error;

/// Why a drama operation failed. The messages are shown to the user.
#[derive(Debug, Error)]
pub enum DramaError {
    #[error("数据库错误：{0}")]
    Db(#[from] DbErr),
    #[error("角色 {0} 不存在")]
    CharacterNotFound(String),
    #[error("{0}")]
    BadReference(String),
    #[error("角色 {character} 已经有{language}参考音")]
    ReferenceExists {
        character: String,
        language: &'static str,
    },
    #[error("世界的名字不能为空")]
    EmptyWorldName,
    #[error("世界 {0} 已经存在")]
    WorldExists(String),
    #[error("世界 {0} 不存在")]
    WorldNotFound(String),
    #[error("角色 {character} 已经在世界 {world} 里")]
    AlreadyCast { world: String, character: String },
    #[error("角色 {character} 不在世界 {world} 里")]
    NotCast { world: String, character: String },
    #[error("世界 {world} 已经有旁白 {narrator}")]
    NarratorTaken { world: String, narrator: String },
    #[error("主要角色的身份不能为空")]
    EmptyDescription,
    #[error("世界 {world} 缺少{missing}")]
    CastIncomplete { world: String, missing: String },
    #[error("这一集不存在")]
    EpisodeNotFound,
    #[error("集 {0} 不在世界 {1} 里")]
    ForeignEpisode(String, String),
    #[error("这一行不存在")]
    LineNotFound,
    #[error("音频为空")]
    EmptyAudio,
}
