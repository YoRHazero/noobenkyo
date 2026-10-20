use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{ComprehensiveVisibility, QuestionType};

/// Settings both lines use.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct QuizSettings {
    /// N: the highest stage. A sense at stage N is mastered.
    pub max_stage: u32,
    /// Question order for words with kanji.
    pub kanji_order: Vec<QuestionType>,
    /// Question order for words without kanji.
    pub kana_order: Vec<QuestionType>,
    pub comprehensive: ComprehensiveVisibility,
}

impl Default for QuizSettings {
    fn default() -> Self {
        use QuestionType::*;
        Self {
            max_stage: 7,
            kanji_order: vec![Sentence, Translation, Reading, Comprehensive],
            kana_order: vec![Sentence, Translation, Comprehensive],
            comprehensive: ComprehensiveVisibility::default(),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum QuizSettingsError {
    #[error("最高档位至少为 1")]
    MaxStage,
    #[error("题型顺序不能为空")]
    EmptyOrder,
    #[error("题型顺序里有重复的题型")]
    DuplicateType,
    #[error("纯假名的词不能出读音题")]
    ReadingForKana,
}

impl QuizSettings {
    pub fn validate(&self) -> Result<(), QuizSettingsError> {
        if self.max_stage == 0 {
            return Err(QuizSettingsError::MaxStage);
        }
        for order in [&self.kanji_order, &self.kana_order] {
            if order.is_empty() {
                return Err(QuizSettingsError::EmptyOrder);
            }
            if order
                .iter()
                .enumerate()
                .any(|(i, t)| order[..i].contains(t))
            {
                return Err(QuizSettingsError::DuplicateType);
            }
        }
        if self.kana_order.contains(&QuestionType::Reading) {
            return Err(QuizSettingsError::ReadingForKana);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use QuestionType::*;

    #[test]
    fn default_is_valid() {
        assert_eq!(QuizSettings::default().validate(), Ok(()));
    }

    #[test]
    fn rejects_bad_settings() {
        let cases = [
            (
                QuizSettings {
                    max_stage: 0,
                    ..QuizSettings::default()
                },
                QuizSettingsError::MaxStage,
            ),
            (
                QuizSettings {
                    kana_order: vec![],
                    ..QuizSettings::default()
                },
                QuizSettingsError::EmptyOrder,
            ),
            (
                QuizSettings {
                    kanji_order: vec![Sentence, Sentence],
                    ..QuizSettings::default()
                },
                QuizSettingsError::DuplicateType,
            ),
            (
                QuizSettings {
                    kana_order: vec![Reading],
                    ..QuizSettings::default()
                },
                QuizSettingsError::ReadingForKana,
            ),
        ];
        for (settings, error) in cases {
            assert_eq!(settings.validate(), Err(error));
        }
    }
}
