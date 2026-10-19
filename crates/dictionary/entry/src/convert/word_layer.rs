use dictionary_word::rules::writing::{WritingError, normalize_writing};

/// The writing, without whitespace.
pub fn writing(raw: &str) -> Result<String, WritingError> {
    normalize_writing(raw)
}

/// Origin and stratum: an omitted field is the empty string.
pub fn optional_text(raw: Option<String>) -> String {
    raw.map(|text| text.trim().to_owned()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn omitted_and_empty_are_the_same() {
        assert_eq!(optional_text(None), "");
        assert_eq!(optional_text(Some(" ".into())), "");
        assert_eq!(optional_text(Some(" coffee ".into())), "coffee");
    }
}
