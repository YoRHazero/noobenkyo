use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder};

use crate::by_key;
use crate::entity::{cast, world};

/// Names of the worlds whose cast has the character, in the order the
/// worlds were created. Shown before deleting the character.
pub async fn character_usage(db: &impl ConnectionTrait, name: &str) -> Result<Vec<String>, DbErr> {
    let Some(found) = by_key::find_character(db, name).await? else {
        return Ok(Vec::new());
    };
    let rows = world::Entity::find()
        .inner_join(cast::Entity)
        .filter(cast::Column::CharacterId.eq(found.id))
        .order_by_asc(world::Column::Id)
        .all(db)
        .await?;
    Ok(rows.into_iter().map(|row| row.name).collect())
}
