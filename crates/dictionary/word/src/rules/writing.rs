//! Normalization and validation of a word's written form.

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WritingError {
    #[error("写法为空")]
    Empty,
    #[error("写法含有半角字符：{0}")]
    HalfWidth(char),
    #[error("写法含有浊点或半浊点符号：{0}")]
    SoundMark(char),
}

/// Removes all whitespace, then rejects half-width characters and
/// standalone (han)dakuten marks.
pub fn normalize_writing(raw: &str) -> Result<String, WritingError> {
    let writing: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
    if writing.is_empty() {
        return Err(WritingError::Empty);
    }
    for c in writing.chars() {
        if is_half_width(c) {
            return Err(WritingError::HalfWidth(c));
        }
        if is_sound_mark(c) {
            return Err(WritingError::SoundMark(c));
        }
    }
    Ok(writing)
}

fn is_half_width(c: char) -> bool {
    c.is_ascii() || matches!(c, '\u{FF61}'..='\u{FFDC}' | '\u{FFE8}'..='\u{FFEE}')
}

fn is_sound_mark(c: char) -> bool {
    matches!(c, '\u{3099}'..='\u{309C}')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_whitespace() {
        assert_eq!(normalize_writing(" 消し　ゴム ").unwrap(), "消しゴム");
    }

    #[test]
    fn rejects_half_width() {
        assert_eq!(
            normalize_writing("Tシャツ").unwrap_err(),
            WritingError::HalfWidth('T')
        );
        assert_eq!(
            normalize_writing("ｺｰﾋｰ").unwrap_err(),
            WritingError::HalfWidth('ｺ')
        );
    }

    #[test]
    fn rejects_sound_marks() {
        assert_eq!(
            normalize_writing("か゛").unwrap_err(),
            WritingError::SoundMark('゛')
        );
        assert_eq!(
            normalize_writing("か\u{3099}").unwrap_err(),
            WritingError::SoundMark('\u{3099}')
        );
    }

    #[test]
    fn rejects_empty() {
        assert_eq!(normalize_writing("　").unwrap_err(), WritingError::Empty);
    }
}
