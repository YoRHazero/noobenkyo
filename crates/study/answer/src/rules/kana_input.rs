const KATAKANA_TO_HIRAGANA_OFFSET: u32 = 0x60;

/// Normalizes a kana answer for comparison: no whitespace, katakana as hiragana.
pub fn normalize_kana_input(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '\u{30A1}'..='\u{30F6}' | 'ヽ' | 'ヾ' => {
                char::from_u32(c as u32 - KATAKANA_TO_HIRAGANA_OFFSET).unwrap_or(c)
            }
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_spaces_and_katakana() {
        assert_eq!(normalize_kana_input(" コー ヒー "), "こーひー");
        assert_eq!(normalize_kana_input("あきらめる"), "あきらめる");
    }
}
