//! The check handed to the model: the preview's warnings written as Chinese
//! lines, for a caller that cannot see the dictionary's types.

use std::collections::BTreeMap;

use crate::{Dropped, ImportOptions, preview};

/// What importing `text` with `options` would merge incompletely or maybe
/// wrongly, one line of Chinese each, in line order: words that look
/// inflected, dropped languages, dropped sentences and unknown fields of the
/// merged entries.
/// Skipped lines are left out. Empty means every line is either merged as
/// written or skipped as a whole.
pub fn check(text: &str, options: ImportOptions) -> Vec<String> {
    let preview = preview(text, options);
    let merged: BTreeMap<u32, &str> = preview
        .entries
        .iter()
        .map(|entry| (entry.line, entry.writing.as_str()))
        .collect();
    let mut unknown: BTreeMap<u32, Vec<&str>> = BTreeMap::new();
    for field in &preview.ignored_fields {
        for line in field.lines.iter().filter(|line| merged.contains_key(line)) {
            unknown.entry(*line).or_default().push(&field.name);
        }
    }

    let mut warnings: Vec<(u32, String)> = Vec::new();
    for line in &preview.inflections {
        warnings.push((*line, "疑似活用形".to_owned()));
    }
    for problem in preview
        .problems
        .iter()
        .filter(|problem| merged.contains_key(&problem.line))
    {
        let what = match &problem.dropped {
            Dropped::OriginLanguage { value } => format!("丢掉语种 {value}：{}", problem.reason),
            Dropped::Sentence { marked } => format!("丢掉例句 {marked}：{}", problem.reason),
            Dropped::Entry | Dropped::Sense => continue,
        };
        warnings.push((problem.line, what));
    }
    for (line, names) in unknown {
        warnings.push((line, format!("未知字段 {}", names.join("、"))));
    }
    // Stable, so a line's warnings keep the order above.
    warnings.sort_by_key(|(line, _)| *line);
    warnings
        .into_iter()
        .map(|(line, what)| format!("{}：{what}", place(line, merged[&line])))
        .collect()
}

fn place(line: u32, writing: &str) -> String {
    if writing.is_empty() {
        format!("第 {line} 行")
    } else {
        format!("第 {line} 行 {writing}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOTHING_CHOSEN: ImportOptions = ImportOptions {
        drop_pure_katakana: false,
        detect_inflections: false,
        keep_progress: true,
    };

    #[test]
    fn lists_the_warnings_of_each_merged_line() {
        let text = concat!(
            r#"{"writing": "諦める", "kana": "あきらめる", "glosses": ["放弃"], "gloss": ["死心"], "pos": "动词", "sentences": [{"marked": "[[諦めない]]で"}]}"#,
            "\n",
            r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"]}"#,
        );
        assert_eq!(
            check(text, NOTHING_CHOSEN),
            [
                "第 1 行 諦める：丢掉例句 [[諦めない]]で：目标词里的汉字 諦 没有注音",
                "第 1 行 諦める：未知字段 gloss、pos",
            ]
        );
    }

    #[test]
    fn lists_a_language_without_an_origin() {
        let text = r#"{"writing": "ネコ", "origin_language": "日语", "kana": "ねこ", "glosses": ["猫"], "sentences": [{"marked": "[[猫]]"}]}"#;
        assert_eq!(
            check(text, NOTHING_CHOSEN),
            [
                "第 1 行 ネコ：丢掉语种 日语：没有词源",
                "第 1 行 ネコ：丢掉例句 [[猫]]：目标词里的汉字 猫 没有注音",
            ]
        );
    }

    #[test]
    fn leaves_out_skipped_lines() {
        let text = concat!(
            "not json\n",
            r#"{"writing": "明日", "glosses": ["明天"], "pos": "名词"}"#,
            "\n",
            r#"{"writing": "パソコン", "pos": "名词"}"#,
        );
        let options = ImportOptions {
            drop_pure_katakana: true,
            ..NOTHING_CHOSEN
        };
        assert!(check(text, options).is_empty());
    }

    #[test]
    fn flags_inflections_when_chosen() {
        let text = r#"{"writing": "食べた", "kana": "たべた", "glosses": ["吃了"]}"#;
        assert!(check(text, NOTHING_CHOSEN).is_empty());
        let options = ImportOptions {
            detect_inflections: true,
            ..NOTHING_CHOSEN
        };
        assert_eq!(check(text, options), ["第 1 行 食べた：疑似活用形"]);
    }

    #[test]
    fn is_empty_for_a_clean_text() {
        let text = r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"]}"#;
        assert!(check(text, ImportOptions::default()).is_empty());
    }
}
