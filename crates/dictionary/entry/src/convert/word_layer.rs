use dictionary_word::rules::writing::{WritingError, normalize_writing};

/// The writing, without whitespace.
pub fn writing(raw: &str) -> Result<String, WritingError> {
    normalize_writing(raw)
}

/// Origin and its language: an omitted field is the empty string.
pub fn optional_text(raw: Option<String>) -> String {
    raw.map(|text| text.trim().to_owned()).unwrap_or_default()
}

/// The language of the origin, which only means something with an origin.
/// Without one it is kept empty, and the value written is handed back to be
/// reported as dropped.
pub fn origin_language(origin: &str, raw: Option<String>) -> (String, Option<String>) {
    let language = optional_text(raw);
    if origin.is_empty() && !language.is_empty() {
        (String::new(), Some(language))
    } else {
        (language, None)
    }
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

    #[test]
    fn a_language_needs_an_origin() {
        assert_eq!(
            origin_language("koffie", Some(" 荷兰语 ".into())),
            ("荷兰语".into(), None)
        );
        assert_eq!(
            origin_language("", Some("日语".into())),
            (String::new(), Some("日语".into()))
        );
        assert_eq!(origin_language("", Some(" ".into())), (String::new(), None));
        assert_eq!(origin_language("koffie", None), (String::new(), None));
    }
}
