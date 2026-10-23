use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter};

use super::{ID_CHUNK, affected};
use crate::LineId;
use crate::entity::line;

/// Deletes lines with their audio; ids already gone are skipped. Returns
/// how many were deleted.
pub async fn delete_lines(db: &impl ConnectionTrait, ids: &[LineId]) -> Result<u32, DbErr> {
    let mut deleted: u64 = 0;
    for chunk in ids.chunks(ID_CHUNK) {
        deleted += line::Entity::delete_many()
            .filter(line::Column::Id.is_in(chunk.iter().map(|id| id.row())))
            .exec(db)
            .await?
            .rows_affected;
    }
    Ok(affected(deleted))
}
