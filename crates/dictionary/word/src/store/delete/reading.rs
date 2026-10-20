use sea_orm::{ConnectionTrait, DbErr};

use crate::ReadingKey;
use crate::store::by_key;
use crate::store::tables::reading;

/// Deletes a reading with its senses, labels and sentences; the word stays.
/// `false` if the reading is already gone.
pub async fn delete_reading(db: &impl ConnectionTrait, key: &ReadingKey) -> Result<bool, DbErr> {
    let Some(found) = by_key::find_reading(db, key).await? else {
        return Ok(false);
    };
    reading::delete_reading(db, found.id).await?;
    Ok(true)
}
