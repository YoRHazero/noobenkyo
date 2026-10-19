use study_quiz::QuestionType;

use super::kana_input::normalize_kana_input;

/// The kana a question expects, if it can be checked: the dictionary-form
/// reading, for translation and reading questions alike.
pub fn expected_answer(question_type: QuestionType, reading_kana: &str) -> Option<String> {
    match question_type {
        QuestionType::Translation | QuestionType::Reading => {
            Some(normalize_kana_input(reading_kana))
        }
        QuestionType::Sentence | QuestionType::Comprehensive => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expects_the_dictionary_form_reading() {
        for question_type in [QuestionType::Translation, QuestionType::Reading] {
            assert_eq!(
                expected_answer(question_type, "あきらめる").as_deref(),
                Some("あきらめる")
            );
        }
        assert_eq!(expected_answer(QuestionType::Sentence, "あ"), None);
        assert_eq!(expected_answer(QuestionType::Comprehensive, "あ"), None);
    }
}
