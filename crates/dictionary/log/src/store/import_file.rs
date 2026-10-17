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
}
