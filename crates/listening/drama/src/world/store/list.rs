use sea_orm::{ConnectionTrait, DbErr, EntityTrait, QueryOrder};

use crate::WorldSummary;
use crate::entity::world;
use crate::episode::store::count_by_world::episode_counts;

/// Every world with its number of episodes, in the order they were created.
pub async fn worlds(db: &impl ConnectionTrait) -> Result<Vec<WorldSummary>, DbErr> {
    let rows = world::Entity::find()
        .order_by_asc(world::Column::Id)
        .all(db)
        .await?;
    let counts = episode_counts(db).await?;
    Ok(rows
        .into_iter()
        .map(|row| WorldSummary {
            episodes: counts.get(&row.id).copied().unwrap_or(0),
            name: row.name,
        })
        .collect())
}
