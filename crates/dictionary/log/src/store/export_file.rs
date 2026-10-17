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
