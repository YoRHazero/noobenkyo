use sea_orm::{ConnectionTrait, DbErr, EntityTrait};

use crate::by_key;
use crate::entity::world;

/// Deletes a world with its cast, episodes, lines and audio. `false` if it
/// is already gone.
pub async fn delete_world(db: &impl ConnectionTrait, name: &str) -> Result<bool, DbErr> {
    let Some(found) = by_key::find_world(db, name).await? else {
        return Ok(false);
    };
    world::Entity::delete_by_id(found.id).exec(db).await?;
    Ok(true)
}
