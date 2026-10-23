use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TransactionTrait};

use crate::DramaError;
use crate::by_key;
use crate::entity::world;
use crate::world::rules::name::clean_name;

/// Creates a world with an empty cast. Its name can never change.
pub async fn create_world(
    db: &DatabaseConnection,
    name: &str,
    setting: &str,
) -> Result<(), DramaError> {
    let name = clean_name(name).ok_or(DramaError::EmptyWorldName)?;
    let txn = db.begin().await?;
    if by_key::find_world(&txn, &name).await?.is_some() {
        return Err(DramaError::WorldExists(name));
    }
    world::ActiveModel {
        name: Set(name),
        setting: Set(setting.trim().to_owned()),
        ..Default::default()
    }
    .insert(&txn)
    .await?;
    txn.commit().await?;
    Ok(())
}
