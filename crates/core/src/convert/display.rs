pub fn to_dictionary(display: study::SentenceDisplay) -> dictionary::SentenceDisplay {
    match display {
        study::SentenceDisplay::KanaOnly => dictionary::SentenceDisplay::KanaOnly,
        study::SentenceDisplay::Blank => dictionary::SentenceDisplay::Blank,
        study::SentenceDisplay::KanjiOnly => dictionary::SentenceDisplay::KanjiOnly,
        study::SentenceDisplay::Full => dictionary::SentenceDisplay::Full,
    }
}
