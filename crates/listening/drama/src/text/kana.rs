/// Whether a character counts as kana in listening: hiragana from ぁ to ゖ,
/// katakana from ァ to ヺ, and ・ ー ゝ ゞ ヽ ヾ. Text-to-speech reads them as
/// they are, so nothing is converted.
pub fn is_kana(c: char) -> bool {
    matches!(c, '\u{3041}'..='\u{3096}' | 'ゝ' | 'ゞ' | '\u{30A1}'..='\u{30FE}')
}

/// Whether the text is not empty and every character is kana.
pub fn is_all_kana(text: &str) -> bool {
    !text.is_empty() && text.chars().all(is_kana)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_hiragana_katakana_and_marks() {
        assert!(is_all_kana("あした"));
        assert!(is_all_kana("コーヒー"));
        assert!(is_all_kana("アリス・リデル"));
        assert!(is_all_kana("いすゞ"));
        assert!(is_all_kana("ヶ"));
    }

    #[test]
    fn rejects_everything_else() {
        assert!(!is_all_kana(""));
        for text in ["明日", "a", "ｱ", "あ い", "゛", "ヿ"] {
            assert!(!is_all_kana(text), "{text}");
        }
    }
}
