//! In-memory database with the log table, plus a sample record.

use chrono::{DateTime, Duration};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::SchemaManager;

use crate::{LogPrediction, LogQuestionType, LogRating, LogRecord, LogSense};

pub async fn memory_db() -> DatabaseConnection {
    let mut options = ConnectOptions::new("sqlite::memory:");
    options
        .max_connections(1)
        .min_connections(1)
        .idle_timeout(None)
        .max_lifetime(None)
        .sqlx_logging(false);
    let db = Database::connect(options).await.unwrap();
    let manager = SchemaManager::new(&db);
    for migration in crate::migrations() {
        migration.up(&manager).await.unwrap();
    }
    db
}

/// A record `seconds` after a fixed moment.
pub fn record(seconds: i64) -> LogRecord {
    let base = DateTime::parse_from_rfc3339("2026-10-17T21:05:02.117+08:00").unwrap();
    LogRecord {
        time: base + Duration::seconds(seconds),
        sense: LogSense {
            writing: "諦める".into(),
            origin: String::new(),
            kana: "あきらめる".into(),
            primary_gloss: "放弃".into(),
        },
        question_type: LogQuestionType::Translation,
        sentence: Some("[[{諦|あきら}めない]]でね。".into()),
        answer: Some("やめる".into()),
        prediction: LogPrediction::Unknown,
        similarity: None,
        embedding_model: None,
        rating: LogRating::Vague,
    }
}
