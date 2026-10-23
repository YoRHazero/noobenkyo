use std::collections::BTreeSet;

use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use super::of_episode::audio_of_episode;
use crate::by_key;
use crate::character::store::languages::languages_by_name;
use crate::entity::line;
use crate::line::rules::voice::voice_problem;
use crate::text::speech::speech_text;
use crate::{DramaError, Language, LineId, SpeechItem};

/// The recordings an episode lacks that its voices can synthesize now, line
/// by line, Japanese before Chinese.
pub async fn missing_audio(
    db: &impl ConnectionTrait,
    episode: Uuid,
) -> Result<Vec<SpeechItem>, DramaError> {
    let found = by_key::require_episode(db, episode).await?;
    let rows = line::Entity::find()
        .filter(line::Column::EpisodeId.eq(found.id))
        .order_by_asc(line::Column::Id)
        .all(db)
        .await?;
    let audio = audio_of_episode(db, found.id).await?;
    let voices: Vec<String> = rows
        .iter()
        .map(|row| row.voice.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let languages = languages_by_name(db, &voices).await?;

    let mut items = Vec::new();
    for row in &rows {
        let present = audio.get(&row.id).map(Vec::as_slice).unwrap_or_default();
        let voice_languages = languages.get(&row.voice).map(Vec::as_slice);
        for language in Language::ALL {
            if present.contains(&language)
                || voice_problem(&row.voice, language, voice_languages).is_some()
            {
                continue;
            }
            items.push(SpeechItem {
                line: LineId(row.id),
                language,
                text: spoken(row, language)?,
                voice: row.voice.clone(),
            });
        }
    }
    Ok(items)
}

fn spoken(row: &line::Model, language: Language) -> Result<String, DbErr> {
    match language {
        Language::Ja => {
            speech_text(&row.ja).map_err(|error| DbErr::Custom(format!("line {}: {error}", row.id)))
        }
        Language::Zh => Ok(row.zh.clone()),
    }
}
