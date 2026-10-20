//! What each question type shows and asks for.

use crate::{
    AnswerInput, Card, ComprehensiveVisibility, QuestionType, Quiz, QuizView, SenseKey,
    SentenceDisplay, SentenceToRender,
};

/// The parts of a quiz that do not depend on its question type.
pub struct QuizParts<'a> {
    pub key: &'a SenseKey,
    pub glosses: &'a [String],
    /// The marked sentence chosen for this quiz.
    pub marked: &'a str,
    pub remaining: u32,
    pub steps: u32,
}

pub fn make_quiz(
    question_type: QuestionType,
    parts: &QuizParts,
    visibility: ComprehensiveVisibility,
) -> Quiz {
    let sentence = |display| {
        Some(SentenceToRender {
            marked: parts.marked.to_owned(),
            display,
        })
    };
    let glosses = || Some(parts.glosses.to_vec());
    let (sentence, glosses, card, answer) = match question_type {
        QuestionType::Sentence => (
            sentence(SentenceDisplay::KanaOnly),
            None,
            None,
            AnswerInput::Chinese,
        ),
        QuestionType::Translation => (
            sentence(SentenceDisplay::Blank),
            glosses(),
            None,
            AnswerInput::Kana {
                max_length: Some(u32::try_from(parts.key.kana.chars().count()).unwrap_or(u32::MAX)),
            },
        ),
        QuestionType::Reading => (
            sentence(SentenceDisplay::KanjiOnly),
            glosses(),
            None,
            AnswerInput::Kana { max_length: None },
        ),
        QuestionType::Comprehensive => (
            None,
            None,
            Some(Card {
                writing: parts.key.writing.clone(),
                kana: parts.key.kana.clone(),
                glosses: parts.glosses.to_vec(),
                visible: visibility,
            }),
            AnswerInput::None,
        ),
    };
    Quiz {
        view: QuizView {
            question_type,
            remaining: parts.remaining,
            steps: parts.steps,
            glosses,
            card,
            answer,
        },
        sentence,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::fixtures::key;

    fn quiz(question_type: QuestionType) -> Quiz {
        let key = key("諦める");
        let glosses = vec!["放弃".to_owned()];
        let parts = QuizParts {
            key: &key,
            glosses: &glosses,
            marked: "[[{諦|あきら}める]]",
            remaining: 2,
            steps: 4,
        };
        make_quiz(question_type, &parts, ComprehensiveVisibility::default())
    }

    #[test]
    fn sentence_question_asks_for_chinese() {
        let quiz = quiz(QuestionType::Sentence);
        assert_eq!(quiz.sentence.unwrap().display, SentenceDisplay::KanaOnly);
        assert_eq!(quiz.view.glosses, None);
        assert_eq!(quiz.view.answer, AnswerInput::Chinese);
        assert_eq!((quiz.view.remaining, quiz.view.steps), (2, 4));
    }

    #[test]
    fn translation_question_limits_input_to_reading_length() {
        let quiz = quiz(QuestionType::Translation);
        assert_eq!(quiz.sentence.unwrap().display, SentenceDisplay::Blank);
        assert_eq!(quiz.view.glosses, Some(vec!["放弃".to_owned()]));
        assert_eq!(
            quiz.view.answer,
            AnswerInput::Kana {
                max_length: Some(5)
            }
        );
    }

    #[test]
    fn reading_question_shows_kanji() {
        let quiz = quiz(QuestionType::Reading);
        assert_eq!(quiz.sentence.unwrap().display, SentenceDisplay::KanjiOnly);
        assert_eq!(quiz.view.answer, AnswerInput::Kana { max_length: None });
    }

    #[test]
    fn comprehensive_question_is_a_card_without_sentence() {
        let quiz = quiz(QuestionType::Comprehensive);
        assert_eq!(quiz.sentence, None);
        let card = quiz.view.card.unwrap();
        assert_eq!(card.writing, "諦める");
        assert!(!card.visible.glosses);
        assert_eq!(quiz.view.answer, AnswerInput::None);
    }
}
