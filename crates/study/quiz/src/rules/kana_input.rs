const KATAKANA_TO_HIRAGANA: u32 = 0x60;

/// A kana answer as compared: no whitespace, katakana as hiragana.
pub fn normalize_kana(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '\u{30A1}'..='\u{30F6}' | 'ヽ' | 'ヾ' => {
                char::from_u32(c as u32 - KATAKANA_TO_HIRAGANA).unwrap_or(c)
            }
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drops_whitespace_and_turns_katakana_into_hiragana() {
        assert_eq!(normalize_kana(" コー\u{3000}ヒー "), "こーひー");
        assert_eq!(normalize_kana("あきらめる"), "あきらめる");
    }
}
