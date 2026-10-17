use chrono::{DateTime, FixedOffset};
use study_answer::Rating;
use study_quiz::QuestionType;

use crate::SenseKey;

/// An answer that meets the logging condition. Whether it is stored is the
/// dictionary's decision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnswerRecord {
    pub time: DateTime<FixedOffset>,
    pub key: SenseKey,
    pub question_type: QuestionType,
    /// The marked sentence shown; `None` for comprehensive questions.
    pub sentence: Option<String>,
    /// The raw input; `None` for comprehensive questions.
    pub answer: Option<String>,
    pub prediction: Option<Rating>,
    pub rating: Rating,
}
