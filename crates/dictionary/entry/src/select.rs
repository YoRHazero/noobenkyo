//! The choices of the import dialog, applied to converted entries.

use dictionary_word::Content;

use crate::filter::inflection::looks_inflected;
use crate::filter::katakana::is_pure_katakana;
use crate::{Converted, ConvertedEntry, Dropped, IgnoredField, ImportOptions, Problem};

/// Converted entries after the import options.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selected {
    pub lines: u32,
    pub entries: Vec<ConvertedEntry>,
    pub problems: Vec<Problem>,
    pub ignored_fields: Vec<IgnoredField>,
    pub skipped_katakana: u32,
}

impl Selected {
    /// The contents to merge, in order.
    pub fn contents(&self) -> Vec<&Content> {
        self.entries.iter().map(|entry| &entry.content).collect()
    }
}

/// Skips pure katakana words (counted), drops words that look inflected
/// (reported), and removes progress unless it is kept.
pub fn select(converted: Converted, options: ImportOptions) -> Selected {
    let mut selected = Selected {
        lines: converted.lines,
        entries: Vec::new(),
        problems: converted.problems,
        ignored_fields: converted.ignored_fields,
        skipped_katakana: 0,
    };
    for mut entry in converted.entries {
        let writing = &entry.content.writing;
        if options.drop_pure_katakana && is_pure_katakana(writing) {
            selected.skipped_katakana += 1;
            continue;
        }
        if options.detect_inflections && looks_inflected(writing) {
            selected.problems.push(Problem {
                line: entry.line,
                writing: entry.writing,
                dropped: Dropped::Entry,
                reason: "疑似活用形".to_owned(),
            });
            continue;
        }
        if !options.keep_progress {
            clear_progress(&mut entry.content);
        }
        selected.entries.push(entry);
    }
    selected.problems.sort_by_key(|problem| problem.line);
    selected
}

fn clear_progress(content: &mut Content) {
    if let Some(sense) = content
        .reading
        .as_mut()
        .and_then(|reading| reading.sense.as_mut())
    {
        sense.progress = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::convert;

    const TEXT: &str = concat!(
        r#"{"writing": "コーヒー", "glosses": ["咖啡"]}"#,
        "\n",
        r#"{"writing": "食べた", "kana": "たべた", "glosses": ["吃了"]}"#,
        "\n",
        r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"], "stage": 3}"#,
    );

    fn writings(selected: &Selected) -> Vec<&str> {
        selected
            .entries
            .iter()
            .map(|e| e.content.writing.as_str())
            .collect()
    }

    #[test]
    fn applies_each_option() {
        let options = ImportOptions {
            drop_pure_katakana: true,
            detect_inflections: true,
            keep_progress: false,
        };
        let selected = select(convert(TEXT), options);
        assert_eq!(writings(&selected), ["明日"]);
        assert_eq!(selected.skipped_katakana, 1);
        assert_eq!(selected.problems.len(), 1);
        assert_eq!(selected.problems[0].line, 2);
        assert_eq!(selected.problems[0].reason, "疑似活用形");
        let sense = selected.entries[0]
            .content
            .reading
            .as_ref()
            .unwrap()
            .sense
            .as_ref();
        assert_eq!(sense.unwrap().progress, None);
    }

    #[test]
    fn keeps_everything_when_nothing_is_chosen() {
        let options = ImportOptions {
            drop_pure_katakana: false,
            detect_inflections: false,
            keep_progress: true,
        };
        let selected = select(convert(TEXT), options);
        assert_eq!(writings(&selected), ["コーヒー", "食べた", "明日"]);
        assert!(selected.problems.is_empty());
        assert_eq!(selected.contents().len(), 3);
    }
}
