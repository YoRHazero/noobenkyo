use sea_orm::{ConnectionTrait, DbErr, EntityTrait};

use crate::entity::line_audio;
use crate::{Language, LineId};

/// A line's audio in `language`; `None` if it has none or the line is gone.
pub async fn audio(
    db: &impl ConnectionTrait,
    id: LineId,
    language: Language,
) -> Result<Option<Vec<u8>>, DbErr> {
    let row = line_audio::Entity::find_by_id((id.row(), language.code().to_owned()))
        .one(db)
        .await?;
    Ok(row.map(|row| row.audio))
}
