use sea_orm::{ConnectionTrait, DbErr};

use crate::SenseKey;
use crate::store::by_key;
use crate::store::tables::sense;

/// Deletes a sense with its labels and sentences; its word and reading stay,
/// even when nothing is left below them. `false` if the sense is already gone.
pub async fn delete_sense(db: &impl ConnectionTrait, key: &SenseKey) -> Result<bool, DbErr> {
    let Some(found) = by_key::find_sense(db, key).await? else {
        return Ok(false);
    };
    sense::delete_sense(db, found.id).await?;
    Ok(true)
}
