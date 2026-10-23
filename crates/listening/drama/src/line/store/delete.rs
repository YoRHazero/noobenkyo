use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter};

use super::ID_CHUNK;
use crate::LineId;
use crate::entity::line;

/// Deletes lines with their audio; ids already gone are skipped. Returns
/// how many were deleted.
pub async fn delete_lines(db: &impl ConnectionTrait, ids: &[LineId]) -> Result<u64, DbErr> {
    let mut deleted = 0;
    for chunk in ids.chunks(ID_CHUNK) {
        deleted += line::Entity::delete_many()
            .filter(line::Column::Id.is_in(chunk.iter().map(|id| id.0)))
            .exec(db)
            .await?
            .rows_affected;
    }
    Ok(deleted)
}
