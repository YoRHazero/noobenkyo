pub fn to_ai(language: listening::Language) -> ai::Language {
    match language {
        listening::Language::Ja => ai::Language::Ja,
        listening::Language::Zh => ai::Language::Zh,
    }
}
