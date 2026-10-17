//! A fixed heuristic for entries of external word lists that may not be in
//! dictionary form. It only flags; the user decides.

use dictionary_word::rules::kanji::has_kanji;

const INFLECTED_ENDINGS: [&str; 10] = [
    "ます",
    "ません",
    "ましょう",
    "ない",
    "た",
    "だ",
    "て",
    "で",
    "ば",
    "たい",
];

/// A word with kanji that ends in one of the fixed inflection endings.
pub fn looks_inflected(writing: &str) -> bool {
    has_kanji(writing)
        && INFLECTED_ENDINGS
            .iter()
            .any(|ending| writing.ends_with(ending))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_inflected_forms() {
        assert!(looks_inflected("食べた"));
        assert!(looks_inflected("行きます"));
        assert!(looks_inflected("諦めない"));
        assert!(!looks_inflected("諦める"));
        assert!(!looks_inflected("明日"));
        assert!(!looks_inflected("あした"));
    }
}
