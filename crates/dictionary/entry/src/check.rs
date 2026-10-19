//! The check handed to the model: what `convert` would take out of a text.

use crate::{Dropped, IgnoredField, Problem, convert};

/// What converting `text` would drop or ignore, one line of Chinese each.
/// Empty means everything in the text is taken as written.
pub fn check(text: &str) -> Vec<String> {
    let converted = convert(text);
    converted
        .problems
        .iter()
        .map(problem_message)
        .chain(converted.ignored_fields.iter().map(ignored_message))
        .collect()
}

fn problem_message(problem: &Problem) -> String {
    let place = if problem.writing.is_empty() {
        format!("第 {} 行", problem.line)
    } else {
        format!("第 {} 行 {}", problem.line, problem.writing)
    };
    let dropped = match &problem.dropped {
        Dropped::Entry => "整条不导入".to_owned(),
        Dropped::Sentence { marked } => format!("丢掉例句 {marked}"),
        Dropped::Sense => "义项没有合并".to_owned(),
    };
    format!("{place}：{dropped}：{}", problem.reason)
}

fn ignored_message(field: &IgnoredField) -> String {
    format!("忽略了不认识的字段 {}（{} 行）", field.name, field.lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_problems_then_ignored_fields() {
        let text = concat!(
            r#"{"writing": "諦める", "kana": "あきらめる", "glosses": ["放弃"], "gloss": ["死心"], "sentences": [{"marked": "[[諦めない]]で"}]}"#,
            "\nnot json\n",
        );
        assert_eq!(
            check(text),
            [
                "第 1 行 諦める：丢掉例句 [[諦めない]]で：目标词里的汉字 諦 没有注音",
                "第 2 行：整条不导入：不是合法的 JSON",
                "忽略了不认识的字段 gloss（1 行）",
            ]
        );
    }

    #[test]
    fn is_empty_for_a_clean_text() {
        assert!(check(r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"]}"#).is_empty());
    }
}
