use ai_invoker::TtsLanguage;

/// The language of a line or a reference recording. A line is spoken with
/// the reference in its own language.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Language {
    Ja,
    Zh,
}

impl Language {
    /// The name the TTS model knows it by.
    pub(crate) fn tts(self) -> TtsLanguage {
        match self {
            Language::Ja => TtsLanguage::Japanese,
            Language::Zh => TtsLanguage::Chinese,
        }
    }

    /// The name used in messages.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Language::Ja => "日语",
            Language::Zh => "中文",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_language_has_its_own_tts_name() {
        assert_eq!(Language::Ja.tts(), TtsLanguage::Japanese);
        assert_eq!(Language::Zh.tts(), TtsLanguage::Chinese);
    }
}
