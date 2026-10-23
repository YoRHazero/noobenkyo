use std::collections::BTreeMap;

use super::entry_content::entry_content;
use super::{Converted, ConvertedEntry};
use crate::read::{ReadLine, read};
use crate::{Dropped, IgnoredField, Problem};

/// Why a language is dropped.
const NO_ORIGIN: &str = "没有词源";

/// Entry text → contents the dictionary understands, plus what was dropped
/// or ignored. A bad sentence drops only itself, and so does a language
/// without an origin; any other problem drops the whole entry.
pub fn convert(text: &str) -> Converted {
    let read = read(text);
    let mut problems = read.problems;
    let mut entries = Vec::new();
    let mut ignored: BTreeMap<String, Vec<u32>> = BTreeMap::new();

    for ReadLine { line, entry } in read.entries {
        for name in entry.unknown_fields() {
            ignored.entry(name).or_default().push(line);
        }
        let writing = entry.writing.clone();
        match entry_content(entry) {
            Ok((content, dropped)) => {
                if let Some(value) = dropped.origin_language {
                    problems.push(Problem {
                        line,
                        writing: writing.clone(),
                        dropped: Dropped::OriginLanguage { value },
                        reason: NO_ORIGIN.to_owned(),
                    });
                }
                problems.extend(dropped.sentences.into_iter().map(|sentence| Problem {
                    line,
                    writing: writing.clone(),
                    dropped: Dropped::Sentence {
                        marked: sentence.marked,
                    },
                    reason: sentence.error.to_string(),
                }));
                entries.push(ConvertedEntry {
                    line,
                    writing,
                    content,
                });
            }
            Err(error) => problems.push(Problem {
                line,
                writing,
                dropped: Dropped::Entry,
                reason: error.to_string(),
            }),
        }
    }

    problems.sort_by_key(|problem| problem.line);
    Converted {
        lines: read.lines,
        entries,
        problems,
        ignored_fields: ignored
            .into_iter()
            .map(|(name, lines)| IgnoredField { name, lines })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = concat!(
        r#"{"writing": "諦める", "kana": "アキラメル", "glosses": ["放弃", " 死心 "], "pos": "动词", "sentences": [{"marked": "{夢|ゆめ}を[[{諦|あきら}めた]]"}, {"marked": "[[諦めない]]で"}]}"#,
        "\n",
        r#"{"writing": "明日", "glosses": ["明天"]}"#,
        "\n",
        r#"{"writing": "Tシャツ", "glosses": ["T恤"]}"#,
        "\nnot json\n\n",
        r#"{"writing": "コーヒー", "glosses": ["咖啡"], "pos": "名词"}"#,
        "\n",
    );

    #[test]
    fn converts_what_it_can_and_reports_the_rest() {
        let converted = convert(TEXT);
        assert_eq!(converted.lines, 5);

        let kept: Vec<(u32, &str)> = converted
            .entries
            .iter()
            .map(|e| (e.line, e.content.writing.as_str()))
            .collect();
        assert_eq!(kept, [(1, "諦める"), (6, "コーヒー")]);
        let sense = converted.entries[0]
            .content
            .reading
            .as_ref()
            .unwrap()
            .sense
            .as_ref()
            .unwrap();
        assert_eq!(sense.glosses, ["放弃", "死心"]);
        assert_eq!(sense.sentences.len(), 1);

        assert_eq!(
            converted.problems,
            [
                Problem {
                    line: 1,
                    writing: "諦める".into(),
                    dropped: Dropped::Sentence {
                        marked: "[[諦めない]]で".into()
                    },
                    reason: "目标词里的汉字 諦 没有注音".into(),
                },
                Problem {
                    line: 2,
                    writing: "明日".into(),
                    dropped: Dropped::Entry,
                    reason: "有汉字的单词要写 kana".into(),
                },
                Problem {
                    line: 3,
                    writing: "Tシャツ".into(),
                    dropped: Dropped::Entry,
                    reason: "写法只能包含汉字和假名，不能有 T".into(),
                },
                Problem {
                    line: 4,
                    writing: String::new(),
                    dropped: Dropped::Entry,
                    reason: "不是合法的 JSON".into(),
                },
            ]
        );
        assert_eq!(
            converted.ignored_fields,
            [IgnoredField {
                name: "pos".into(),
                lines: vec![1, 6]
            }]
        );
    }
}
