use dictionary_word::rules::kana::KanaError;
use dictionary_word::rules::writing::WritingError;
use thiserror::Error;

/// Why a whole entry is dropped. The messages are shown to the user.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum EntryError {
    #[error("{0}")]
    Writing(#[from] WritingError),
    #[error("{0}")]
    Kana(#[from] KanaError),
    #[error("有汉字的单词要写 kana")]
    MissingKana,
    #[error("没有汉字的单词，kana 应为 {expected}")]
    KanaMismatch { expected: String },
    #[error("有 labels、sentences 或进度时必须有 glosses")]
    MissingGlosses,
}
