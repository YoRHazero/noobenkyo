//! Judging a submitted answer by fixed rules.

use super::kana_input::normalize_kana;
use crate::{Judgement, QuestionType};

/// Kana questions are correct when the input is the reading; Chinese
/// questions only when it is one of the glosses, and unsure otherwise.
/// Empty input is wrong. Comprehensive questions take no input: `None`.
pub fn judge(
    question_type: QuestionType,
    input: &str,
    kana: &str,
    glosses: &[String],
) -> Option<Judgement> {
    match question_type {
        QuestionType::Comprehensive => None,
        QuestionType::Translation | QuestionType::Reading => {
            let input = normalize_kana(input);
            Some(if !input.is_empty() && input == normalize_kana(kana) {
                Judgement::Correct
            } else {
                Judgement::Wrong
            })
        }
        QuestionType::Sentence => {
            let input = without_whitespace(input);
            Some(if input.is_empty() {
                Judgement::Wrong
            } else if glosses.iter().any(|g| without_whitespace(g) == input) {
                Judgement::Correct
            } else {
                Judgement::Unsure
            })
        }
    }
}

fn without_whitespace(text: &str) -> String {
    text.chars().filter(|c| !c.is_whitespace()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn glosses() -> Vec<String> {
        vec!["放弃".into(), "死心".into()]
    }

    #[test]
    fn kana_answers_are_right_or_wrong() {
        for question_type in [QuestionType::Translation, QuestionType::Reading] {
            let judge = |input| judge(question_type, input, "あきらめる", &glosses());
            assert_eq!(judge("アキラメル"), Some(Judgement::Correct));
            assert_eq!(judge("あきら める"), Some(Judgement::Correct));
            assert_eq!(judge("あきらねる"), Some(Judgement::Wrong));
            assert_eq!(judge(" "), Some(Judgement::Wrong));
        }
    }

    #[test]
    fn chinese_answers_are_right_only_when_a_gloss() {
        let judge = |input| judge(QuestionType::Sentence, input, "あきらめる", &glosses());
        assert_eq!(judge(" 死心 "), Some(Judgement::Correct));
        assert_eq!(judge("不放弃"), Some(Judgement::Unsure));
        assert_eq!(judge("放"), Some(Judgement::Unsure));
        assert_eq!(judge(""), Some(Judgement::Wrong));
    }

    #[test]
    fn comprehensive_answers_are_not_judged() {
        assert_eq!(
            judge(QuestionType::Comprehensive, "", "あ", &glosses()),
            None
        );
    }
}
