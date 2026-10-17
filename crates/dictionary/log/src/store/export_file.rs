use std::io::Write;
use std::path::Path;

use sea_orm::{ConnectionTrait, EntityTrait, QueryOrder};

use crate::LogError;
use crate::entity::log;
use crate::rules::row::from_row;

/// Writes all records as JSON Lines in time order; returns how many.
pub async fn export_file(db: &impl ConnectionTrait, path: &Path) -> Result<u32, LogError> {
    let rows = log::Entity::find()
        .order_by_asc(log::Column::Time)
        .all(db)
        .await?;
    let mut file = std::io::BufWriter::new(std::fs::File::create(path)?);
    let mut written = 0u32;
    for record in rows.into_iter().filter_map(from_row) {
        serde_json::to_writer(&mut file, &record).map_err(std::io::Error::other)?;
        file.write_all(b"\n")?;
        written += 1;
    }
    file.flush()?;
    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::append::append;
    use crate::store::test_db::{memory_db, record};

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
