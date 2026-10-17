use std::path::Path;

use sea_orm::{DatabaseConnection, TransactionTrait};

use crate::store::append::append;
use crate::{LogError, LogImportReport, LogRecord};

/// Adds the records of a JSON Lines file; records with an existing key are skipped.
pub async fn import_file(
    db: &DatabaseConnection,
    path: &Path,
) -> Result<LogImportReport, LogError> {
    let content = std::fs::read_to_string(path)?;
    let mut report = LogImportReport::default();
    let txn = db.begin().await?;
    for line in content.lines().filter(|line| !line.trim().is_empty()) {
        report.lines += 1;
        match serde_json::from_str::<LogRecord>(line) {
            Ok(record) => report.added += u32::from(append(&txn, &record).await?),
            Err(_) => report.invalid += 1,
        }
    }
    txn.commit().await?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::append::append;
    use crate::store::count::count;
    use crate::store::export_file::export_file;
    use crate::store::test_db::{memory_db, record};

    #[tokio::test]
    async fn exports_and_imports_as_a_union() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("log.jsonl");

        let source = memory_db().await;
        append(&source, &record(0)).await.unwrap();
        append(&source, &record(1)).await.unwrap();
        assert_eq!(export_file(&source, &path).await.unwrap(), 2);

        let target = memory_db().await;
        append(&target, &record(1)).await.unwrap();
        append(&target, &record(2)).await.unwrap();
        let mut content = std::fs::read_to_string(&path).unwrap();
        content.push_str("not json\n");
        std::fs::write(&path, content).unwrap();

        let report = import_file(&target, &path).await.unwrap();
        assert_eq!(
            report,
            LogImportReport {
                lines: 3,
                added: 1,
                invalid: 1
            }
        );
        assert_eq!(count(&target).await.unwrap(), 3);
    }

    #[tokio::test]
    async fn exported_line_matches_design_format() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("log.jsonl");
        let db = memory_db().await;
        append(&db, &record(0)).await.unwrap();
        export_file(&db, &path).await.unwrap();
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            "{\"时间\":\"2026-10-17T21:05:02.117+08:00\",\"义项\":{\"写法\":\"諦める\",\"词源\":\"\",\"读音\":\"あきらめる\",\"主释义\":\"放弃\"},\"题型\":\"翻译题\",\"例句\":\"[[{諦|あきら}めない]]でね。\",\"原始答案\":\"やめる\",\"预判\":\"不会\",\"自评\":\"模糊\"}\n"
        );
    }
}
