//! Kana classification and katakana-to-hiragana conversion.

use thiserror::Error;

const KATAKANA_TO_HIRAGANA_OFFSET: u32 = 0x60;

pub fn is_hiragana(c: char) -> bool {
    matches!(c, '\u{3041}'..='\u{3096}' | 'ゝ' | 'ゞ' | 'ー')
}

pub fn is_katakana(c: char) -> bool {
    matches!(c, '\u{30A1}'..='\u{30FA}' | 'ヽ' | 'ヾ' | 'ー')
}

pub fn is_kana(c: char) -> bool {
    is_hiragana(c) || is_katakana(c)
}

/// Converts katakana to hiragana, leaving every other character unchanged.
pub fn to_hiragana(text: &str) -> String {
    text.chars().map(char_to_hiragana).collect()
}

fn char_to_hiragana(c: char) -> char {
    match c {
        '\u{30A1}'..='\u{30F6}' | 'ヽ' | 'ヾ' => {
            char::from_u32(c as u32 - KATAKANA_TO_HIRAGANA_OFFSET).unwrap_or(c)
        }
        _ => c,
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum KanaError {
    #[error("读音为空")]
    Empty,
    #[error("读音含有假名以外的字符：{0}")]
    NotKana(char),
}

/// Normalizes a reading: removes whitespace and converts it to hiragana.
pub fn normalize_reading(raw: &str) -> Result<String, KanaError> {
    let kana = to_hiragana(
        &raw.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>(),
    );
    if kana.is_empty() {
        return Err(KanaError::Empty);
    }
    match kana.chars().find(|c| !is_hiragana(*c)) {
        Some(c) => Err(KanaError::NotKana(c)),
        None => Ok(kana),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_katakana_and_keeps_long_vowel_mark() {
        assert_eq!(to_hiragana("コーヒー"), "こーひー");
        assert_eq!(to_hiragana("アルバイト"), "あるばいと");
        assert_eq!(to_hiragana("いう"), "いう");
    }

    #[test]
    fn normalizes_reading() {
        assert_eq!(normalize_reading(" アス ").unwrap(), "あす");
        assert_eq!(normalize_reading("").unwrap_err(), KanaError::Empty);
        assert_eq!(
            normalize_reading("あa").unwrap_err(),
            KanaError::NotKana('a')
        );
        assert_eq!(
            normalize_reading("明日").unwrap_err(),
            KanaError::NotKana('明')
        );
    }
}
