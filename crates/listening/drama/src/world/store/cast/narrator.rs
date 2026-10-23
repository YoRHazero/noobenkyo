use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

use crate::DramaError;
use crate::entity::{cast, character, world};

/// Fails when the world's narrator is a character other than `character_id`.
pub async fn check_narrator_free(
    db: &impl ConnectionTrait,
    world: &world::Model,
    character_id: i64,
) -> Result<(), DramaError> {
    let narrator = cast::Entity::find()
        .filter(cast::Column::WorldId.eq(world.id))
        .filter(cast::Column::Role.eq("narrator"))
        .find_also_related(character::Entity)
        .one(db)
        .await?;
    match narrator {
        Some((row, Some(found))) if row.character_id != character_id => {
            Err(DramaError::NarratorTaken {
                world: world.name.clone(),
                narrator: found.name,
            })
        }
        _ => Ok(()),
    }
}
