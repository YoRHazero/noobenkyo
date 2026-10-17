use serde::Serialize;
use study_quiz::{AnswerInput, Card, QuestionType, SentenceDisplay};

use crate::Zone;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Question {
    pub view: QuestionView,
    /// The sentence to render for the question, if it shows one.
    pub sentence: Option<SentenceToRender>,
}

/// What the frontend shows for a question, apart from the rendered sentence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct QuestionView {
    pub zone: Zone,
    pub question_type: QuestionType,
    /// Glosses shown below the sentence.
    pub glosses: Option<Vec<String>>,
    /// The face of a comprehensive question.
    pub card: Option<Card>,
    pub answer_input: AnswerInput,
    pub remaining: u32,
    pub steps: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SentenceToRender {
    pub marked: String,
    pub display: SentenceDisplay,
}
