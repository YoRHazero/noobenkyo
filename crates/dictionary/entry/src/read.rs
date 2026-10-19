//! Splits an entry text into lines and reads each one as an `Entry`.

use serde_json::Value;

use crate::{Dropped, Entry, Problem};

/// The entries of a text, with what could not be read.
pub struct Read {
    /// Non-empty lines.
    pub lines: u32,
    pub entries: Vec<ReadLine>,
    pub problems: Vec<Problem>,
}

pub struct ReadLine {
    pub line: u32,
    pub entry: Entry,
}

/// Skips blank lines. A line that is not JSON, or whose fields have the
/// wrong shape, is dropped as a whole.
pub fn read(text: &str) -> Read {
    let text = text.strip_prefix('\u{FEFF}').unwrap_or(text);
    let mut read = Read {
        lines: 0,
        entries: Vec::new(),
        problems: Vec::new(),
    };
    for (index, raw) in text.lines().enumerate() {
        if raw.trim().is_empty() {
            continue;
        }
        read.lines += 1;
        let line = u32::try_from(index + 1).unwrap_or(u32::MAX);
        match read_line(raw) {
            Ok(entry) => read.entries.push(ReadLine { line, entry }),
            Err((writing, reason)) => read.problems.push(Problem {
                line,
                writing,
                dropped: Dropped::Entry,
                reason,
            }),
        }
    }
    read
}

/// The entry, or the writing (if any) and the reason it cannot be read.
fn read_line(raw: &str) -> Result<Entry, (String, String)> {
    let value: Value =
        serde_json::from_str(raw).map_err(|_| (String::new(), "不是合法的 JSON".to_owned()))?;
    let writing = value
        .get("writing")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();
    serde_json::from_value(value).map_err(|error| (writing, format!("字段不对：{error}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_lines_with_numbers_and_skips_blank_ones() {
        let read = read("\u{FEFF}{\"writing\": \"明日\"}\n\n  \r\n{\"writing\": \"上\"}\r\n");
        assert_eq!(read.lines, 2);
        let lines: Vec<u32> = read.entries.iter().map(|e| e.line).collect();
        assert_eq!(lines, [1, 4]);
        assert_eq!(read.entries[1].entry.writing, "上");
        assert!(read.problems.is_empty());
    }

    #[test]
    fn drops_lines_that_cannot_be_read() {
        let read = read(concat!(
            "not json\n",
            "{\"writing\": \"明日\", \"stage\": \"2\"}\n",
            "{\"kana\": \"あす\"}\n",
            "[1, 2]\n",
        ));
        assert!(read.entries.is_empty());
        let problems: Vec<(u32, &str, bool)> = read
            .problems
            .iter()
            .map(|p| (p.line, p.writing.as_str(), p.dropped == Dropped::Entry))
            .collect();
        assert_eq!(
            problems,
            [
                (1, "", true),
                (2, "明日", true),
                (3, "", true),
                (4, "", true)
            ]
        );
        assert_eq!(read.problems[0].reason, "不是合法的 JSON");
        assert!(read.problems[1].reason.starts_with("字段不对："));
        assert!(read.problems[2].reason.contains("writing"));
    }
}
