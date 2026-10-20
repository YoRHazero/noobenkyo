use serde::Serialize;

use crate::{AnswerInput, Card, QuestionType, SentenceToRender};

/// A quiz: what the frontend shows, and the sentence for core to render.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Quiz {
    pub view: QuizView,
    /// `None` for comprehensive questions, which show no sentence.
    pub sentence: Option<SentenceToRender>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct QuizView {
    pub question_type: QuestionType,
    /// Steps left, and steps in the whole question order.
    pub remaining: u32,
    pub steps: u32,
    /// Glosses shown below the sentence.
    pub glosses: Option<Vec<String>>,
    /// The face of a comprehensive question.
    pub card: Option<Card>,
    pub answer: AnswerInput,
}
