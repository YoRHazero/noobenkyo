use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogPrediction {
    #[serde(rename = "会")]
    Known,
    #[serde(rename = "模糊")]
    Vague,
    #[serde(rename = "不会")]
    Unknown,
    #[serde(rename = "无")]
    Absent,
}

impl LogPrediction {
    const ALL: [Self; 4] = [Self::Known, Self::Vague, Self::Unknown, Self::Absent];

    /// The text stored in the database and written to files.
    pub fn text(self) -> &'static str {
        match self {
            Self::Known => "会",
            Self::Vague => "模糊",
            Self::Unknown => "不会",
            Self::Absent => "无",
        }
    }

    pub fn from_text(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|value| value.text() == text)
    }
}
