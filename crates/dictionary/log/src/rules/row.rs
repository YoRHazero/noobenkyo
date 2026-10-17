//! Conversion between records and table rows.

use chrono::{DateTime, FixedOffset, SubsecRound};
use sea_orm::Set;

use crate::entity::log;
use crate::{LogPrediction, LogQuestionType, LogRating, LogRecord, LogSense};

/// Timestamps are kept to the millisecond so the same moment always has the same key.
pub fn key_time(time: DateTime<FixedOffset>) -> DateTime<FixedOffset> {
    time.trunc_subsecs(3)
}

pub fn to_row(record: &LogRecord) -> log::ActiveModel {
    log::ActiveModel {
        time: Set(key_time(record.time)),
        writing: Set(record.sense.writing.clone()),
        origin: Set(record.sense.origin.clone()),
        kana: Set(record.sense.kana.clone()),
        primary_gloss: Set(record.sense.primary_gloss.clone()),
        question_type: Set(record.question_type.text().to_owned()),
        sentence: Set(record.sentence.clone()),
        answer: Set(record.answer.clone()),
        prediction: Set(record.prediction.text().to_owned()),
        similarity: Set(record.similarity),
        embedding_model: Set(record.embedding_model.clone()),
        rating: Set(record.rating.text().to_owned()),
    }
}

/// `None` if a stored text is not a known value.
pub fn from_row(row: log::Model) -> Option<LogRecord> {
    Some(LogRecord {
        time: row.time,
        question_type: LogQuestionType::from_text(&row.question_type)?,
        prediction: LogPrediction::from_text(&row.prediction)?,
        rating: LogRating::from_text(&row.rating)?,
        sense: LogSense {
            writing: row.writing,
            origin: row.origin,
            kana: row.kana,
            primary_gloss: row.primary_gloss,
        },
        sentence: row.sentence,
        answer: row.answer,
        similarity: row.similarity,
        embedding_model: row.embedding_model,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_time_to_milliseconds() {
        let time = DateTime::parse_from_rfc3339("2026-10-17T21:05:02.117999+08:00").unwrap();
        assert_eq!(key_time(time).to_rfc3339(), "2026-10-17T21:05:02.117+08:00");
    }

    #[test]
    fn parses_design_example() {
        let line = r#"{"时间": "2026-10-17T21:05:02.117+08:00", "义项": {"写法": "諦める", "词源": "", "读音": "あきらめる", "主释义": "放弃"}, "题型": "翻译题", "例句": "{何度|なんど}{失敗|しっぱい}しても、[[{諦|あきら}めない]]でね。", "原始答案": "やめる", "预判": "不会", "自评": "模糊"}"#;
        let record: LogRecord = serde_json::from_str(line).unwrap();
        assert_eq!(record.question_type, LogQuestionType::Translation);
        assert_eq!(record.prediction, LogPrediction::Unknown);
        assert_eq!(record.rating, LogRating::Vague);
        assert_eq!(record.similarity, None);
    }
}
