use sea_orm::{ConnectionTrait, DbErr, EntityTrait};

use crate::by_key;
use crate::entity::cast;

/// Takes a character out of a world's cast. Lines it voiced keep its name.
/// `false` if it was not in the cast.
pub async fn remove_cast(
    db: &impl ConnectionTrait,
    world: &str,
    character: &str,
) -> Result<bool, DbErr> {
    let (Some(found_world), Some(found_character)) = (
        by_key::find_world(db, world).await?,
        by_key::find_character(db, character).await?,
    ) else {
        return Ok(false);
    };
    let deleted = cast::Entity::delete_by_id((found_world.id, found_character.id))
        .exec(db)
        .await?;
    Ok(deleted.rows_affected > 0)
}
