use chrono::{DateTime, FixedOffset};
use sea_orm::sea_query::{Expr, ExprTrait};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QuerySelect,
    Set,
};
use uuid::Uuid;

use crate::entity::episode;
use crate::episode::rules::number::next_number;

/// Adds an episode without lines to a world, numbered after its largest number.
pub async fn insert_episode(
    db: &impl ConnectionTrait,
    world_id: i64,
    uuid: Uuid,
    title: &str,
    summary: &str,
    created_at: DateTime<FixedOffset>,
) -> Result<episode::Model, DbErr> {
    let largest: Option<Option<f64>> = episode::Entity::find()
        .select_only()
        .column_as(
            Expr::col((episode::Entity, episode::Column::Number)).max(),
            "largest",
        )
        .filter(episode::Column::WorldId.eq(world_id))
        .into_tuple()
        .one(db)
        .await?;
    episode::ActiveModel {
        uuid: Set(uuid),
        world_id: Set(world_id),
        number: Set(next_number(largest.flatten())),
        title: Set(title.to_owned()),
        summary: Set(summary.to_owned()),
        created_at: Set(created_at),
        ..Default::default()
    }
    .insert(db)
    .await
}
