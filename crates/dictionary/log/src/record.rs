use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{LogPrediction, LogQuestionType, LogRating, LogSense};

/// One answer log record; also one line of the log file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogRecord {
    #[serde(rename = "时间")]
    pub time: DateTime<FixedOffset>,
    #[serde(rename = "义项")]
    pub sense: LogSense,
    #[serde(rename = "题型")]
    pub question_type: LogQuestionType,
    /// The marked sentence shown; absent for comprehensive questions.
    #[serde(rename = "例句", default, skip_serializing_if = "Option::is_none")]
    pub sentence: Option<String>,
    /// What the user typed; absent for comprehensive questions.
    #[serde(rename = "原始答案", default, skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    #[serde(rename = "预判")]
    pub prediction: LogPrediction,
    #[serde(rename = "相似度", default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<f64>,
    #[serde(rename = "预判模型", default, skip_serializing_if = "Option::is_none")]
    pub embedding_model: Option<String>,
    #[serde(rename = "自评")]
    pub rating: LogRating,
}
