use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogQuestionType {
    #[serde(rename = "例句题")]
    Sentence,
    #[serde(rename = "翻译题")]
    Translation,
    #[serde(rename = "读音题")]
    Reading,
    #[serde(rename = "综合题")]
    Comprehensive,
}

impl LogQuestionType {
    const ALL: [Self; 4] = [
        Self::Sentence,
        Self::Translation,
        Self::Reading,
        Self::Comprehensive,
    ];

    /// The text stored in the database and written to files.
    pub fn text(self) -> &'static str {
        match self {
            Self::Sentence => "例句题",
            Self::Translation => "翻译题",
            Self::Reading => "读音题",
            Self::Comprehensive => "综合题",
        }
    }

    pub fn from_text(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|value| value.text() == text)
    }
}
