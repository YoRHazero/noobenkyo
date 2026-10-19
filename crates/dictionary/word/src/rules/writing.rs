//! Normalization and validation of a word's written form.

use thiserror::Error;

use super::kana::is_kana;
use super::kanji::is_kanji;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WritingError {
    #[error("写法为空")]
    Empty,
    #[error("写法只能包含汉字和假名，不能有 {0}")]
    NotAllowed(char),
}

/// Removes all whitespace; what remains may only be kanji and kana.
pub fn normalize_writing(raw: &str) -> Result<String, WritingError> {
    let writing: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
    if writing.is_empty() {
        return Err(WritingError::Empty);
    }
    match writing.chars().find(|c| !is_kanji(*c) && !is_kana(*c)) {
        Some(c) => Err(WritingError::NotAllowed(c)),
        None => Ok(writing),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_whitespace() {
        assert_eq!(normalize_writing(" 消し　ゴム ").unwrap(), "消しゴム");
    }

    #[test]
    fn keeps_kanji_and_kana() {
        for writing in ["諦める", "一ヶ月", "人々", "コーヒー", "きれい"] {
            assert_eq!(normalize_writing(writing).unwrap(), writing);
        }
    }

    #[test]
    fn rejects_letters_digits_and_symbols() {
        for (writing, c) in [
            ("Tシャツ", 'T'),
            ("Ｔシャツ", 'Ｔ'),
            ("2日", '2'),
            ("コカ・コーラ", '・'),
            ("明日！", '！'),
        ] {
            assert_eq!(
                normalize_writing(writing).unwrap_err(),
                WritingError::NotAllowed(c),
                "{writing}"
            );
        }
    }

    #[test]
    fn rejects_half_width_katakana_and_sound_marks() {
        assert_eq!(
            normalize_writing("ｺｰﾋｰ").unwrap_err(),
            WritingError::NotAllowed('ｺ')
        );
        assert_eq!(
            normalize_writing("か゛").unwrap_err(),
            WritingError::NotAllowed('゛')
        );
        assert_eq!(
            normalize_writing("か\u{3099}").unwrap_err(),
            WritingError::NotAllowed('\u{3099}')
        );
    }

    #[test]
    fn rejects_empty() {
        assert_eq!(normalize_writing("　").unwrap_err(), WritingError::Empty);
    }
}
