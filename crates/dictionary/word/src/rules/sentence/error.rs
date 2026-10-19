use thiserror::Error;

/// Why a marked sentence is invalid. The messages are shown to the user.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SentenceError {
    #[error("注音没有闭合")]
    UnclosedRuby,
    #[error("注音格式应为 {{汉字|かな}}")]
    MalformedRuby,
    #[error("多余的 {0}")]
    StrayMark(char),
    #[error("目标词没有闭合")]
    UnclosedTarget,
    #[error("多余的 ]]")]
    StrayTargetClose,
    #[error("目标词不能嵌套")]
    NestedTarget,
    #[error("目标词为空")]
    EmptyTarget,
    #[error("目标词必须恰好出现一次，实际出现 {0} 次")]
    TargetCount(usize),
    #[error("目标词里的汉字 {0} 没有注音")]
    TargetKanjiWithoutRuby(char),
    #[error("注音 {0} 只能包含假名和 ー")]
    RubyNotKana(String),
}
