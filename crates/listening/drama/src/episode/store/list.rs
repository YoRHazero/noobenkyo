use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder};

use crate::by_key;
use crate::entity::episode;
use crate::line::store::of_episode::line_counts;
use crate::{DramaError, EpisodeSummary};

/// A world's episodes by number; equal numbers in the order they were created.
pub async fn episodes(
    db: &impl ConnectionTrait,
    world: &str,
) -> Result<Vec<EpisodeSummary>, DramaError> {
    let found = by_key::require_world(db, world).await?;
    let rows = episode::Entity::find()
        .filter(episode::Column::WorldId.eq(found.id))
        .order_by_asc(episode::Column::Number)
        .order_by_asc(episode::Column::Id)
        .all(db)
        .await?;
    let counts = line_counts(db, found.id).await?;
    Ok(rows
        .into_iter()
        .map(|row| {
            let count = counts.get(&row.id).copied().unwrap_or_default();
            EpisodeSummary {
                uuid: row.uuid,
                number: row.number,
                title: row.title,
                created_at: row.created_at,
                lines: count.lines,
                missing_ja: count.missing_ja,
                missing_zh: count.missing_zh,
            }
        })
        .collect())
}
