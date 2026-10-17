use super::kanji::has_kanji;
use crate::{QuestionType, QuizSettings};

/// The question order for a word: words with kanji and words without have their own.
pub fn order_for<'a>(settings: &'a QuizSettings, writing: &str) -> &'a [QuestionType] {
    if has_kanji(writing) {
        &settings.kanji_order
    } else {
        &settings.kana_order
    }
}

/// With `remaining` steps left, the question is the remaining-th from the end
/// of the order. `remaining` is clamped to 1..=len.
pub fn question_type_for(order: &[QuestionType], remaining: u32) -> QuestionType {
    let len = order.len();
    let remaining = usize::try_from(remaining).unwrap_or(len).clamp(1, len);
    order[len - remaining]
}

#[cfg(test)]
mod tests {
    use super::*;
    use QuestionType::*;

    #[test]
    fn picks_order_by_kanji() {
        let settings = QuizSettings::default();
        assert_eq!(order_for(&settings, "諦める").len(), 4);
        assert_eq!(
            order_for(&settings, "きれい"),
            [Sentence, Translation, Comprehensive]
        );
    }

    #[test]
    fn counts_from_the_end() {
        let order = [Sentence, Translation, Reading, Comprehensive];
        assert_eq!(question_type_for(&order, 4), Sentence);
        assert_eq!(question_type_for(&order, 2), Reading);
        assert_eq!(question_type_for(&order, 1), Comprehensive);
        assert_eq!(question_type_for(&order, 9), Sentence);
        assert_eq!(question_type_for(&order, 0), Comprehensive);
    }
}
