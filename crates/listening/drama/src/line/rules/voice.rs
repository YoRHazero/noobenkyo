use crate::Language;

/// Why a line's voice cannot synthesize `language` now, if it can't: the
/// character must still exist and have a reference recording in it.
/// `languages` are the character's reference languages, `None` when it is gone.
pub fn voice_problem(
    voice: &str,
    language: Language,
    languages: Option<&[Language]>,
) -> Option<String> {
    match languages {
        None => Some(format!("声音 {voice} 已经被删除")),
        Some(languages) if !languages.contains(&language) => {
            Some(format!("声音 {voice} 没有{}参考音", language.name()))
        }
        Some(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn needs_the_character_and_its_reference() {
        assert_eq!(
            voice_problem("美咲", Language::Ja, Some(&[Language::Ja])),
            None
        );
        assert_eq!(
            voice_problem("美咲", Language::Zh, Some(&[Language::Ja])),
            Some("声音 美咲 没有中文参考音".into())
        );
        assert_eq!(
            voice_problem("美咲", Language::Ja, None),
            Some("声音 美咲 已经被删除".into())
        );
    }
}
