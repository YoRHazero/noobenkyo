use sea_orm::{ConnectionTrait, DbErr, EntityTrait};
use uuid::Uuid;

use crate::by_key;
use crate::entity::episode;

/// Deletes an episode with its lines and audio. `false` if it is already gone.
pub async fn delete_episode(db: &impl ConnectionTrait, uuid: Uuid) -> Result<bool, DbErr> {
    let Some(found) = by_key::find_episode(db, uuid).await? else {
        return Ok(false);
    };
    episode::Entity::delete_by_id(found.id).exec(db).await?;
    Ok(true)
}
