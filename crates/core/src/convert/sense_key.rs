pub fn to_study(key: dictionary::SenseKey) -> study::SenseKey {
    study::SenseKey {
        writing: key.writing,
        origin: key.origin,
        kana: key.kana,
        primary_gloss: key.primary_gloss,
    }
}

pub fn to_dictionary(key: study::SenseKey) -> dictionary::SenseKey {
    dictionary::SenseKey {
        writing: key.writing,
        origin: key.origin,
        kana: key.kana,
        primary_gloss: key.primary_gloss,
    }
}
