use std::collections::HashMap;

use sea_orm::sea_query::{Expr, ExprTrait};
use sea_orm::{ConnectionTrait, DbErr, EntityTrait, QuerySelect};

use crate::entity::episode;

/// How many episodes each world has, by world id. Worlds without episodes
/// are left out.
pub async fn episode_counts(db: &impl ConnectionTrait) -> Result<HashMap<i64, u32>, DbErr> {
    let rows: Vec<(i64, i64)> = episode::Entity::find()
        .select_only()
        .column(episode::Column::WorldId)
        .column_as(
            Expr::col((episode::Entity, episode::Column::Id)).count(),
            "episodes",
        )
        .group_by(episode::Column::WorldId)
        .into_tuple()
        .all(db)
        .await?;
    Ok(rows
        .into_iter()
        .map(|(world_id, count)| (world_id, u32::try_from(count).unwrap_or(0)))
        .collect())
}
