//! Kana classification, readings, and katakana-to-hiragana conversion.

use thiserror::Error;

const KATAKANA_TO_HIRAGANA_OFFSET: u32 = 0x60;

/// Hiragana as it may appear in a writing.
pub fn is_hiragana(c: char) -> bool {
    matches!(c, '\u{3041}'..='\u{3096}' | 'ゝ' | 'ゞ' | 'ー')
}

/// Katakana as it may appear in a writing.
pub fn is_katakana(c: char) -> bool {
    matches!(c, '\u{30A1}'..='\u{30FA}' | 'ヽ' | 'ヾ' | 'ー')
}

/// Kana as it may appear in a writing.
pub fn is_kana(c: char) -> bool {
    is_hiragana(c) || is_katakana(c)
}

/// Kana a reading may consist of: hiragana from ぁ to ゔ, their katakana
/// counterparts from ァ to ヴ, and the long vowel mark.
pub fn is_reading_kana(c: char) -> bool {
    matches!(c, '\u{3041}'..='\u{3094}' | '\u{30A1}'..='\u{30F4}' | 'ー')
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
    #[error("读音只能包含假名和 ー，不能有 {0}")]
    NotKana(char),
}

/// Normalizes a reading: removes whitespace, checks every character is
/// reading kana, and converts katakana to hiragana.
pub fn normalize_reading(raw: &str) -> Result<String, KanaError> {
    let kana: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
    if kana.is_empty() {
        return Err(KanaError::Empty);
    }
    match kana.chars().find(|c| !is_reading_kana(*c)) {
        Some(c) => Err(KanaError::NotKana(c)),
        None => Ok(to_hiragana(&kana)),
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
        assert_eq!(normalize_reading("ヴァイオリン").unwrap(), "ゔぁいおりん");
        assert_eq!(normalize_reading("がっこう").unwrap(), "がっこう");
    }

    #[test]
    fn rejects_other_characters() {
        assert_eq!(normalize_reading("").unwrap_err(), KanaError::Empty);
        for (raw, c) in [
            ("あa", 'a'),
            ("明日", '明'),
            ("いすゞ", 'ゞ'),
            ("こゝろ", 'ゝ'),
            ("こか・こーら", '・'),
            ("ヶ", 'ヶ'),
            ("ｱｽ", 'ｱ'),
        ] {
            assert_eq!(
                normalize_reading(raw).unwrap_err(),
                KanaError::NotKana(c),
                "{raw}"
            );
        }
    }
}
