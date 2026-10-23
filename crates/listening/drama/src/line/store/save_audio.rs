use sea_orm::sea_query::OnConflict;
use sea_orm::{ConnectionTrait, EntityTrait, Set};

use crate::by_key;
use crate::entity::line_audio;
use crate::{DramaError, Language, LineId};

/// Stores a line's audio in `language`, replacing any it had. Fails when
/// the line has been deleted, so the caller can skip it.
pub async fn save_audio(
    db: &impl ConnectionTrait,
    id: LineId,
    language: Language,
    wav: &[u8],
) -> Result<(), DramaError> {
    if wav.is_empty() {
        return Err(DramaError::EmptyAudio);
    }
    let found = by_key::require_line(db, id).await?;
    line_audio::Entity::insert(line_audio::ActiveModel {
        line_id: Set(found.id),
        language: Set(language.code().to_owned()),
        audio: Set(wav.to_vec()),
    })
    .on_conflict(
        OnConflict::columns([line_audio::Column::LineId, line_audio::Column::Language])
            .update_column(line_audio::Column::Audio)
            .to_owned(),
    )
    .exec(db)
    .await?;
    Ok(())
}
