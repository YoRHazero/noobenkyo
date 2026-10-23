use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, TransactionTrait};

use super::narrator::check_narrator_free;
use crate::by_key;
use crate::entity::cast;
use crate::world::rules::role::clean_role;
use crate::{DramaError, Role};

/// Puts a character in a world's cast. A world has at most one narrator.
pub async fn add_cast(
    db: &DatabaseConnection,
    world: &str,
    character: &str,
    role: &Role,
) -> Result<(), DramaError> {
    let role = clean_role(role).ok_or(DramaError::EmptyDescription)?;
    let txn = db.begin().await?;
    let found_world = by_key::require_world(&txn, world).await?;
    let found_character = by_key::require_character(&txn, character).await?;
    let existing = cast::Entity::find_by_id((found_world.id, found_character.id))
        .one(&txn)
        .await?;
    if existing.is_some() {
        return Err(DramaError::AlreadyCast {
            world: found_world.name,
            character: found_character.name,
        });
    }
    if role == Role::Narrator {
        check_narrator_free(&txn, &found_world, found_character.id).await?;
    }
    let (code, description) = role.columns();
    cast::ActiveModel {
        world_id: Set(found_world.id),
        character_id: Set(found_character.id),
        role: Set(code.to_owned()),
        description: Set(description),
    }
    .insert(&txn)
    .await?;
    txn.commit().await?;
    Ok(())
}
