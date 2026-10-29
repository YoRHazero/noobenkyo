use crate::convert::language;

/// A character's reference in one language, as the sample its voice is
/// cloned from.
pub fn to_ai(
    voice: &str,
    language: listening::Language,
    reference: listening::Reference,
) -> ai::VoiceSample {
    ai::VoiceSample {
        voice: voice.to_owned(),
        language: language::to_ai(language),
        audio: reference.audio,
        text: reference.text,
    }
}
