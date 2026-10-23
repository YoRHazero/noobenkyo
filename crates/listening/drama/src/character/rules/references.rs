use std::collections::HashSet;

use crate::{Language, NewReference};

/// What is wrong with one reference recording, if anything.
pub fn reference_problems(language: Language, audio: &[u8], text: &str) -> Vec<String> {
    let mut problems = Vec::new();
    if audio.is_empty() {
        problems.push(format!("{}参考音的音频为空", language.name()));
    }
    if text.trim().is_empty() {
        problems.push(format!("{}参考音的文字为空", language.name()));
    }
    problems
}

/// What is wrong with a new character's references: there is at least one,
/// at most one per language, and each is complete.
pub fn references_problems(references: &[NewReference]) -> Vec<String> {
    if references.is_empty() {
        return vec!["至少要有一段参考音".to_owned()];
    }
    let mut seen = HashSet::new();
    let mut problems = Vec::new();
    for reference in references {
        if !seen.insert(reference.language) {
            problems.push(format!("{}参考音重复", reference.language.name()));
            continue;
        }
        problems.extend(reference_problems(
            reference.language,
            &reference.audio,
            &reference.text,
        ));
    }
    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference(language: Language, audio: &[u8], text: &str) -> NewReference {
        NewReference {
            language,
            audio: audio.to_vec(),
            text: text.into(),
        }
    }

    #[test]
    fn one_complete_reference_is_enough() {
        assert!(references_problems(&[reference(Language::Zh, b"wav", "你好")]).is_empty());
    }

    #[test]
    fn needs_at_least_one() {
        assert_eq!(references_problems(&[]), vec!["至少要有一段参考音"]);
    }

    #[test]
    fn lists_every_problem() {
        let problems = references_problems(&[
            reference(Language::Ja, b"", " "),
            reference(Language::Zh, b"wav", "你好"),
            reference(Language::Zh, b"wav", "你好"),
        ]);
        assert_eq!(
            problems,
            vec![
                "日语参考音的音频为空",
                "日语参考音的文字为空",
                "中文参考音重复"
            ]
        );
    }
}
