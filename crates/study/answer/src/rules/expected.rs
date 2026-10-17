use study_quiz::QuestionType;

use super::kana_input::normalize_kana_input;

/// The kana a question expects, if it can be checked: the dictionary-form
/// reading for translation questions, the target's kana in the sentence for
/// reading questions.
pub fn expected_answer(
    question_type: QuestionType,
    reading_kana: &str,
    target_kana: &str,
) -> Option<String> {
    match question_type {
        QuestionType::Translation => Some(normalize_kana_input(reading_kana)),
        QuestionType::Reading => Some(normalize_kana_input(target_kana)),
        QuestionType::Sentence | QuestionType::Comprehensive => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expects_reading_or_target_kana() {
        assert_eq!(
            expected_answer(QuestionType::Translation, "あきらめる", "あきらめない").as_deref(),
            Some("あきらめる")
        );
        assert_eq!(
            expected_answer(QuestionType::Reading, "あきらめる", "アキラメナイ").as_deref(),
            Some("あきらめない")
        );
        assert_eq!(expected_answer(QuestionType::Sentence, "あ", "あ"), None);
        assert_eq!(
            expected_answer(QuestionType::Comprehensive, "あ", "あ"),
            None
        );
    }
}
