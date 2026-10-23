use sea_orm::{ConnectionTrait, DbErr, EntityTrait};
use uuid::Uuid;

use crate::EpisodeDetail;
use crate::by_key;
use crate::entity::world;
use crate::line::store::of_episode::lines_of;

/// One episode with its lines; `None` if it is gone.
pub async fn episode(
    db: &impl ConnectionTrait,
    uuid: Uuid,
) -> Result<Option<EpisodeDetail>, DbErr> {
    let Some(found) = by_key::find_episode(db, uuid).await? else {
        return Ok(None);
    };
    let owner = world::Entity::find_by_id(found.world_id)
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("episode world".into()))?;
    Ok(Some(EpisodeDetail {
        lines: lines_of(db, found.id).await?,
        uuid: found.uuid,
        world: owner.name,
        number: found.number,
        title: found.title,
        summary: found.summary,
        created_at: found.created_at,
    }))
}
