use crate::Language;

/// A reference recording to clone. Passed once per voice and language, and
/// shared by every item spoken in them: an episode has dozens of items but
/// only a few voices.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VoiceSample {
    pub voice: String,
    pub language: Language,
    /// A WAV file's bytes.
    pub audio: Vec<u8>,
    /// What the recording says.
    pub text: String,
}

/// The sample of `voice` in `language`.
pub(crate) fn find<'a>(
    samples: &'a [VoiceSample],
    voice: &str,
    language: Language,
) -> Option<&'a VoiceSample> {
    samples
        .iter()
        .find(|sample| sample.voice == voice && sample.language == language)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(voice: &str, language: Language) -> VoiceSample {
        VoiceSample {
            voice: voice.into(),
            language,
            audio: vec![1],
            text: format!("{voice} {language:?}"),
        }
    }

    #[test]
    fn matches_both_voice_and_language() {
        let samples = [
            sample("美咲", Language::Ja),
            sample("健二", Language::Zh),
            sample("美咲", Language::Zh),
        ];
        let found = find(&samples, "美咲", Language::Zh).unwrap();
        assert_eq!(found.text, "美咲 Zh");
        assert!(find(&samples, "健二", Language::Ja).is_none());
    }
}
