//! In-memory database and fixtures for store tests.

use chrono::{DateTime, FixedOffset};
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use sea_orm_migration::SchemaManager;

use crate::Progress;
use crate::store::{reading, sense, word};

pub async fn memory_db() -> DatabaseConnection {
    let mut options = ConnectOptions::new("sqlite::memory:");
    options
        .max_connections(1)
        .min_connections(1)
        .idle_timeout(None)
        .max_lifetime(None)
        .sqlx_logging(false)
        .map_sqlx_sqlite_opts(|opts| opts.foreign_keys(true));
    let db = Database::connect(options).await.unwrap();
    let manager = SchemaManager::new(&db);
    for migration in crate::migrations() {
        migration.up(&manager).await.unwrap();
    }
    db
}

pub fn time(text: &str) -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339(text).unwrap()
}

pub fn progress_at(stage: u32, passes: u32, lapses: u32) -> Progress {
    Progress {
        stage,
        passes,
        lapses,
        modified_at: time("2026-10-17T21:05:02.117+08:00"),
    }
}

/// Returns the id of the reading, creating the word and reading as needed.
pub async fn reading_of(db: &impl ConnectionTrait, writing: &str, kana: &str) -> i64 {
    let word = match word::find_word(db, writing, "").await.unwrap() {
        Some(word) => word,
        None => word::insert_word(db, writing, "", "").await.unwrap(),
    };
    match reading::find_reading(db, word.id, kana).await.unwrap() {
        Some(reading) => reading.id,
        None => reading::insert_reading(db, word.id, kana).await.unwrap().id,
    }
}

/// Creates a sense with no progress and returns its id.
pub async fn sense_of(db: &impl ConnectionTrait, writing: &str, kana: &str, gloss: &str) -> i64 {
    let reading = reading_of(db, writing, kana).await;
    sense::insert_sense(db, reading, gloss, &[], &progress_at(0, 0, 0))
        .await
        .unwrap()
        .id
}
