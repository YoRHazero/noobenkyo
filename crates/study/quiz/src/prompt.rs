use crate::{AnswerInput, Card, QuestionType, SentenceDisplay};

/// What a question shows and asks for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prompt {
    pub question_type: QuestionType,
    /// How to show the sentence; `None` when no sentence is shown.
    pub sentence_display: Option<SentenceDisplay>,
    /// Glosses shown below the sentence.
    pub glosses: Option<Vec<String>>,
    /// The face of a comprehensive question.
    pub card: Option<Card>,
    pub answer_input: AnswerInput,
}
