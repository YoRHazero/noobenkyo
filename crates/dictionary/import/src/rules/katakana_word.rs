use dictionary_word::rules::kana::is_katakana;

/// Whether the writing consists only of katakana (the middle dot counts).
pub fn is_pure_katakana(writing: &str) -> bool {
    !writing.is_empty() && writing.chars().all(|c| is_katakana(c) || c == '・')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_pure_katakana() {
        assert!(is_pure_katakana("コーヒー"));
        assert!(is_pure_katakana("ソフト・ウェア"));
        assert!(!is_pure_katakana("消しゴム"));
        assert!(!is_pure_katakana("きれい"));
    }
}
