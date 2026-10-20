use study_quiz::QuizSettings;

use crate::LearnSettings;

/// Whether new settings empty the learning zone: its capacity or a question
/// order changed.
pub fn clears_zone(
    old: (&QuizSettings, &LearnSettings),
    new: (&QuizSettings, &LearnSettings),
) -> bool {
    old.1.capacity != new.1.capacity
        || old.0.kanji_order != new.0.kanji_order
        || old.0.kana_order != new.0.kana_order
}

#[cfg(test)]
mod tests {
    use super::*;
    use study_quiz::QuestionType;

    #[test]
    fn only_capacity_and_orders_clear() {
        let (quiz, learn) = (QuizSettings::default(), LearnSettings::default());
        let bigger = LearnSettings {
            capacity: 20,
            ..learn.clone()
        };
        assert!(clears_zone((&quiz, &learn), (&quiz, &bigger)));
        let shorter = QuizSettings {
            kanji_order: vec![QuestionType::Comprehensive],
            ..quiz.clone()
        };
        assert!(clears_zone((&quiz, &learn), (&shorter, &learn)));
        let other = (
            QuizSettings {
                max_stage: 3,
                ..quiz.clone()
            },
            LearnSettings {
                recent_limit: 5,
                ..learn.clone()
            },
        );
        assert!(!clears_zone((&quiz, &learn), (&other.0, &other.1)));
    }
}
