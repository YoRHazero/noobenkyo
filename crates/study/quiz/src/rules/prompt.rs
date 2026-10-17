//! What each question type shows and asks for.

use crate::{AnswerInput, Card, ComprehensiveVisibility, Prompt, QuestionType, SentenceDisplay};

/// The sense as a question sees it.
pub struct SenseFace<'a> {
    pub writing: &'a str,
    pub kana: &'a str,
    pub glosses: &'a [String],
}

pub fn build_prompt(
    question_type: QuestionType,
    sense: &SenseFace,
    visibility: ComprehensiveVisibility,
) -> Prompt {
    let glosses = || Some(sense.glosses.to_vec());
    match question_type {
        QuestionType::Sentence => Prompt {
            question_type,
            sentence_display: Some(SentenceDisplay::KanaOnly),
            glosses: None,
            card: None,
            answer_input: AnswerInput::Chinese,
        },
        QuestionType::Translation => Prompt {
            question_type,
            sentence_display: Some(SentenceDisplay::Blank),
            glosses: glosses(),
            card: None,
            answer_input: AnswerInput::Kana {
                max_length: Some(u32::try_from(sense.kana.chars().count()).unwrap_or(u32::MAX)),
            },
        },
        QuestionType::Reading => Prompt {
            question_type,
            sentence_display: Some(SentenceDisplay::KanjiOnly),
            glosses: glosses(),
            card: None,
            answer_input: AnswerInput::Kana { max_length: None },
        },
        QuestionType::Comprehensive => Prompt {
            question_type,
            sentence_display: None,
            glosses: None,
            card: Some(Card {
                writing: sense.writing.to_owned(),
                kana: sense.kana.to_owned(),
                glosses: sense.glosses.to_vec(),
                visible: visibility,
            }),
            answer_input: AnswerInput::None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prompt_for(question_type: QuestionType) -> Prompt {
        let glosses = vec!["放弃".to_owned()];
        let sense = SenseFace {
            writing: "諦める",
            kana: "あきらめる",
            glosses: &glosses,
        };
        build_prompt(question_type, &sense, ComprehensiveVisibility::default())
    }

    #[test]
    fn sentence_question_hides_glosses() {
        let prompt = prompt_for(QuestionType::Sentence);
        assert_eq!(prompt.sentence_display, Some(SentenceDisplay::KanaOnly));
        assert_eq!(prompt.glosses, None);
        assert_eq!(prompt.answer_input, AnswerInput::Chinese);
    }

    #[test]
    fn translation_question_limits_input_to_reading_length() {
        let prompt = prompt_for(QuestionType::Translation);
        assert_eq!(prompt.sentence_display, Some(SentenceDisplay::Blank));
        assert_eq!(prompt.glosses, Some(vec!["放弃".to_owned()]));
        assert_eq!(
            prompt.answer_input,
            AnswerInput::Kana {
                max_length: Some(5)
            }
        );
    }

    #[test]
    fn reading_question_shows_kanji() {
        let prompt = prompt_for(QuestionType::Reading);
        assert_eq!(prompt.sentence_display, Some(SentenceDisplay::KanjiOnly));
        assert_eq!(prompt.answer_input, AnswerInput::Kana { max_length: None });
    }

    #[test]
    fn comprehensive_question_is_a_card() {
        let prompt = prompt_for(QuestionType::Comprehensive);
        assert_eq!(prompt.sentence_display, None);
        let card = prompt.card.unwrap();
        assert!(!card.visible.glosses && card.visible.writing && card.visible.kana);
        assert_eq!(prompt.answer_input, AnswerInput::None);
    }
}
