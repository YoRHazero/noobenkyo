/// Whether the writing contains kanji (CJK ideographs plus 々, 〆 and 〇).
pub fn has_kanji(writing: &str) -> bool {
    writing.chars().any(|c| {
        matches!(
            c,
            '\u{3400}'..='\u{4DBF}'
                | '\u{4E00}'..='\u{9FFF}'
                | '\u{F900}'..='\u{FAFF}'
                | '\u{20000}'..='\u{2FA1F}'
                | '々'
                | '〆'
                | '〇'
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_kanji() {
        assert!(has_kanji("諦める"));
        assert!(has_kanji("々"));
        assert!(!has_kanji("コーヒー"));
    }
}
