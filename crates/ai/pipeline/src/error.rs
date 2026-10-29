use ai_invoker::InvokeError;
use thiserror::Error;

use crate::Language;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PipelineError {
    #[error(transparent)]
    Invoke(#[from] InvokeError),
    /// An item's voice and language have no sample among those passed.
    #[error("没有 {voice} 的{}参考音", .language.name())]
    NoSample { voice: String, language: Language },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_the_missing_sample() {
        let error = PipelineError::NoSample {
            voice: "美咲".into(),
            language: Language::Zh,
        };
        assert_eq!(error.to_string(), "没有 美咲 的中文参考音");
    }
}
