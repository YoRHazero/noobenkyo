use sea_orm::{ConnectionTrait, DbErr, EntityTrait};

use crate::by_key;
use crate::entity::character;

/// Deletes a character with its references and its places in every cast.
/// Lines it voiced keep its name. `false` if it is already gone.
pub async fn delete_character(db: &impl ConnectionTrait, name: &str) -> Result<bool, DbErr> {
    let Some(found) = by_key::find_character(db, name).await? else {
        return Ok(false);
    };
    character::Entity::delete_by_id(found.id).exec(db).await?;
    Ok(true)
}
