use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};

use crate::DramaError;
use crate::by_key;
use crate::entity::world;

/// Replaces a world's setting. Episodes already written keep theirs.
pub async fn update_setting(
    db: &impl ConnectionTrait,
    name: &str,
    setting: &str,
) -> Result<(), DramaError> {
    let found = by_key::require_world(db, name).await?;
    world::ActiveModel {
        id: Set(found.id),
        setting: Set(setting.trim().to_owned()),
        ..Default::default()
    }
    .update(db)
    .await?;
    Ok(())
}
