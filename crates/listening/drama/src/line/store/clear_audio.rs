use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter};

use super::ID_CHUNK;
use crate::entity::line_audio;
use crate::{Language, LineId};

/// Drops the lines' audio in `language`, the first step of synthesizing it
/// again. Returns how many recordings were dropped.
pub async fn clear_audio(
    db: &impl ConnectionTrait,
    ids: &[LineId],
    language: Language,
) -> Result<u64, DbErr> {
    let mut cleared = 0;
    for chunk in ids.chunks(ID_CHUNK) {
        cleared += line_audio::Entity::delete_many()
            .filter(line_audio::Column::LineId.is_in(chunk.iter().map(|id| id.0)))
            .filter(line_audio::Column::Language.eq(language.code()))
            .exec(db)
            .await?
            .rows_affected;
    }
    Ok(cleared)
}
