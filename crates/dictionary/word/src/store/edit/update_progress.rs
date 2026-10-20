use sea_orm::{ConnectionTrait, DbErr};

use crate::store::by_key;
use crate::store::tables::sense;
use crate::{Progress, SenseKey};

/// Overwrites the progress of a sense. `false` if the sense is gone.
pub async fn update_progress(
    db: &impl ConnectionTrait,
    key: &SenseKey,
    progress: &Progress,
) -> Result<bool, DbErr> {
    let Some(model) = by_key::find_sense(db, key).await? else {
        return Ok(false);
    };
    sense::set_progress(db, model.id, progress).await?;
    Ok(true)
}
