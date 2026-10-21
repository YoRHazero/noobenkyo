//! What an entry text would bring in under the import options, before it
//! meets the dictionary. Importing merges exactly these entries.

use dictionary_word::Content;
use serde::Serialize;

use crate::filter::inflection::looks_inflected;
use crate::filter::katakana::is_pure_katakana;
use crate::{ConvertedEntry, IgnoredField, ImportOptions, Problem, convert};

/// Converted entries after the import options.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct ImportPreview {
    /// Non-empty lines in the text.
    pub lines: u32,
    /// What would be merged, in text order.
    pub entries: Vec<ConvertedEntry>,
    /// Pure katakana words the options skip, in text order.
    pub skipped_katakana: Vec<ConvertedEntry>,
    /// Lines of entries that look inflected. They are merged like the
    /// others; the options only flag them.
    pub inflections: Vec<u32>,
    /// What was dropped, by line.
    pub problems: Vec<Problem>,
    /// Unknown fields, by name.
    pub ignored_fields: Vec<IgnoredField>,
}

impl ImportPreview {
    /// The contents to merge, in order.
    pub fn contents(&self) -> Vec<&Content> {
        self.entries.iter().map(|entry| &entry.content).collect()
    }
}

/// Converts the text, then skips pure katakana words, flags words that look
/// inflected, and removes progress unless it is kept.
pub fn preview(text: &str, options: ImportOptions) -> ImportPreview {
    let converted = convert(text);
    let mut preview = ImportPreview {
        lines: converted.lines,
        entries: Vec::new(),
        skipped_katakana: Vec::new(),
        inflections: Vec::new(),
        problems: converted.problems,
        ignored_fields: converted.ignored_fields,
    };
    for mut entry in converted.entries {
        let writing = &entry.content.writing;
        if options.drop_pure_katakana && is_pure_katakana(writing) {
            preview.skipped_katakana.push(entry);
            continue;
        }
        if options.detect_inflections && looks_inflected(writing) {
            preview.inflections.push(entry.line);
        }
        if !options.keep_progress {
            clear_progress(&mut entry.content);
        }
        preview.entries.push(entry);
    }
    preview.problems.sort_by_key(|problem| problem.line);
    preview
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

    const TEXT: &str = concat!(
        r#"{"writing": "コーヒー", "glosses": ["咖啡"]}"#,
        "\n",
        r#"{"writing": "食べた", "kana": "たべた", "glosses": ["吃了"]}"#,
        "\n",
        r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"], "stage": 3}"#,
    );

    fn lines_and_writings(entries: &[ConvertedEntry]) -> Vec<(u32, &str)> {
        entries
            .iter()
            .map(|e| (e.line, e.content.writing.as_str()))
            .collect()
    }

    #[test]
    fn applies_each_option() {
        let options = ImportOptions {
            drop_pure_katakana: true,
            detect_inflections: true,
            keep_progress: false,
        };
        let preview = preview(TEXT, options);
        assert_eq!(
            lines_and_writings(&preview.entries),
            [(2, "食べた"), (3, "明日")]
        );
        assert_eq!(
            lines_and_writings(&preview.skipped_katakana),
            [(1, "コーヒー")]
        );
        assert_eq!(preview.inflections, [2]);
        assert!(preview.problems.is_empty());
        let sense = preview.entries[1]
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
        let preview = preview(TEXT, options);
        assert_eq!(
            lines_and_writings(&preview.entries),
            [(1, "コーヒー"), (2, "食べた"), (3, "明日")]
        );
        assert!(preview.skipped_katakana.is_empty());
        assert!(preview.inflections.is_empty());
        assert!(preview.problems.is_empty());
        assert_eq!(preview.contents().len(), 3);
    }
}
