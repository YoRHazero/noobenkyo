use std::io::Write;
use std::path::Path;

use dictionary_word::store::{label, reading, sense, sentence, word};
use sea_orm::{ConnectionTrait, DbErr};
use thiserror::Error;

use crate::ExportReport;
use crate::rules::build_entries::{Rows, build_entries};

#[derive(Debug, Error)]
pub enum ExportError {
    #[error("写入文件失败：{0}")]
    Io(#[from] std::io::Error),
    #[error("数据库错误：{0}")]
    Db(#[from] DbErr),
}

/// Writes every word, reading and sense as JSON Lines.
pub async fn export_file(
    db: &impl ConnectionTrait,
    path: &Path,
    keep_progress: bool,
) -> Result<ExportReport, ExportError> {
    let rows = Rows {
        words: word::all_words(db).await?,
        readings: reading::all_readings(db).await?,
        senses: sense::all_senses(db).await?,
        labels: label::all_labels(db).await?,
        sentences: sentence::all_sentences(db).await?,
    };
    let entries = build_entries(&rows, keep_progress);
    let mut file = std::io::BufWriter::new(std::fs::File::create(path)?);
    for entry in &entries {
        serde_json::to_writer(&mut file, entry).map_err(std::io::Error::other)?;
        file.write_all(b"\n")?;
    }
    file.flush()?;
    Ok(ExportReport {
        entries: u32::try_from(entries.len()).unwrap_or(u32::MAX),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;
    use dictionary_import::{ImportOptions, import_file};
    use sea_orm::{ConnectOptions, Database, DatabaseConnection};
    use sea_orm_migration::SchemaManager;

    async fn memory_db() -> DatabaseConnection {
        let mut options = ConnectOptions::new("sqlite::memory:");
        options
            .max_connections(1)
            .min_connections(1)
            .idle_timeout(None)
            .max_lifetime(None)
            .sqlx_logging(false);
        let db = Database::connect(options).await.unwrap();
        let manager = SchemaManager::new(&db);
        for migration in dictionary_word::migrations() {
            migration.up(&manager).await.unwrap();
        }
        db
    }

    #[tokio::test]
    async fn round_trips_through_import() {
        let source = "\
{\"writing\":\"諦める\",\"stratum\":\"和语\",\"kana\":\"あきらめる\",\"glosses\":[\"放弃\",\"死心\"],\"labels\":[\"N3\"],\"sentences\":[{\"marked\":\"[[{諦|あきら}めない]]で\",\"translation\":\"别放弃\"}],\"stage\":2,\"passes\":3,\"lapses\":1,\"modified_at\":\"2026-10-17T21:05:02.117+08:00\"}
{\"writing\":\"パソコン\",\"origin\":\"personal computer\"}
";
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("in.jsonl");
        let output = dir.path().join("out.jsonl");
        std::fs::write(&input, source).unwrap();

        let db = memory_db().await;
        let now = DateTime::parse_from_rfc3339("2026-10-18T09:00:00+08:00").unwrap();
        let options = ImportOptions {
            drop_pure_katakana: false,
            ..ImportOptions::default()
        };
        import_file(&db, &input, options, now, &mut |_| {})
            .await
            .unwrap();

        let report = export_file(&db, &output, true).await.unwrap();
        assert_eq!(report.entries, 2);
        assert_eq!(std::fs::read_to_string(&output).unwrap(), source);
    }
}
