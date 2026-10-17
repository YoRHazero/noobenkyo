//! Imports a JSON Lines file in one transaction.

use std::path::Path;

use chrono::{DateTime, FixedOffset};
use sea_orm::{DatabaseConnection, DbErr, TransactionTrait};
use thiserror::Error;

use super::merge::merge_entry;
use super::merge_outcome::{SenseOutcome, tally};
use crate::rules::inflection::looks_inflected;
use crate::rules::katakana_word::is_pure_katakana;
use crate::rules::validate::validate;
use crate::{Entry, ImportOptions, ImportProgress, ImportReport, ImportWarning};

/// How often progress is reported, in entries.
const PROGRESS_STEP: u32 = 100;

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("读取文件失败：{0}")]
    Io(#[from] std::io::Error),
    #[error("数据库错误：{0}")]
    Db(#[from] DbErr),
}

pub async fn import_file(
    db: &DatabaseConnection,
    path: &Path,
    options: ImportOptions,
    now: DateTime<FixedOffset>,
    on_progress: &mut (dyn FnMut(ImportProgress) + Send),
) -> Result<ImportReport, ImportError> {
    let content = std::fs::read_to_string(path)?;
    let lines: Vec<(u32, &str)> = content
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(index, line)| (u32::try_from(index + 1).unwrap_or(u32::MAX), line))
        .collect();
    let total = u32::try_from(lines.len()).unwrap_or(u32::MAX);
    let mut report = ImportReport {
        entries: total,
        ..ImportReport::default()
    };

    let txn = db.begin().await?;
    for (done, (line, text)) in lines.into_iter().enumerate() {
        import_line(&txn, line, text, options, now, &mut report).await?;
        let done = u32::try_from(done + 1).unwrap_or(u32::MAX);
        if done % PROGRESS_STEP == 0 {
            on_progress(ImportProgress { done, total });
        }
    }
    txn.commit().await?;
    on_progress(ImportProgress { done: total, total });
    Ok(report)
}

async fn import_line(
    db: &impl sea_orm::ConnectionTrait,
    line: u32,
    text: &str,
    options: ImportOptions,
    now: DateTime<FixedOffset>,
    report: &mut ImportReport,
) -> Result<(), DbErr> {
    let warn = |writing: &str, message: String| ImportWarning {
        line,
        writing: writing.to_owned(),
        message,
    };
    let entry: Entry = match serde_json::from_str(text) {
        Ok(entry) => entry,
        Err(error) => {
            report.warnings.push(warn("", format!("格式错误：{error}")));
            return Ok(());
        }
    };
    let raw_writing = entry.writing.clone();
    let entry = match validate(entry) {
        Ok(entry) => entry,
        Err(error) => {
            report.warnings.push(warn(&raw_writing, error.to_string()));
            return Ok(());
        }
    };
    if options.drop_pure_katakana && is_pure_katakana(&entry.writing) {
        report.skipped_katakana += 1;
        return Ok(());
    }
    if options.detect_inflections && looks_inflected(&entry.writing) {
        report
            .warnings
            .push(warn(&entry.writing, "疑似活用形，未导入".to_owned()));
        return Ok(());
    }
    let outcome = merge_entry(db, &entry, options.keep_progress, now).await?;
    tally(report, &outcome);
    if let SenseOutcome::Conflict(primaries) = &outcome.sense {
        report.warnings.push(warn(
            &entry.writing,
            format!("释义和多个已有义项有交集：{}", primaries.join("、")),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::test_db::memory_db;
    use dictionary_word::store::sense::all_senses;
    use std::io::Write;

    fn now() -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339("2026-10-18T09:00:00+08:00").unwrap()
    }

    fn file(lines: &[&str]) -> tempfile::NamedTempFile {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        writeln!(file, "{}", lines.join("\n")).unwrap();
        file
    }

    #[tokio::test]
    async fn imports_lines_and_collects_warnings() {
        let db = memory_db().await;
        let file = file(&[
            r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"]}"#,
            "",
            r#"{"writing": "明日", "kana": "あす", "glosses": ["明天"]}"#,
            r#"{"writing": "コーヒー", "glosses": ["咖啡"]}"#,
            r#"not json"#,
            r#"{"writing": "食べた", "kana": "たべた", "glosses": ["吃了"]}"#,
            r#"{"writing": "明日", "glosses": ["明天"]}"#,
        ]);
        let options = ImportOptions {
            detect_inflections: true,
            ..ImportOptions::default()
        };
        let mut updates = Vec::new();
        let report = import_file(&db, file.path(), options, now(), &mut |p| updates.push(p))
            .await
            .unwrap();

        assert_eq!(report.entries, 6);
        assert_eq!(report.words_added, 1);
        assert_eq!(report.readings_added, 2);
        assert_eq!(report.senses_added, 2);
        assert_eq!(report.skipped_katakana, 1);
        let lines: Vec<u32> = report.warnings.iter().map(|w| w.line).collect();
        assert_eq!(lines, [5, 6, 7]);
        assert_eq!(report.warnings[2].message, "有汉字的单词要写 kana");
        assert_eq!(updates.last(), Some(&ImportProgress { done: 6, total: 6 }));
        assert_eq!(all_senses(&db).await.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn keeps_katakana_words_when_asked() {
        let db = memory_db().await;
        let file = file(&[r#"{"writing": "コーヒー", "origin": "coffee", "glosses": ["咖啡"]}"#]);
        let options = ImportOptions {
            drop_pure_katakana: false,
            ..ImportOptions::default()
        };
        let report = import_file(&db, file.path(), options, now(), &mut |_| {})
            .await
            .unwrap();
        assert_eq!(report.senses_added, 1);
        assert_eq!(report.skipped_katakana, 0);
    }
}
