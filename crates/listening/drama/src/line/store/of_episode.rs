//! Reading the lines of episodes, and counting them.

use std::collections::HashMap;

use sea_orm::sea_query::{Expr, ExprTrait, Func};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, JoinType, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait,
};

use crate::entity::{episode, line, line_audio};
use crate::{Language, LineId, LineView};

/// How many lines an episode has, how many characters they hold, and how
/// many lack audio in each language.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LineCounts {
    pub lines: u32,
    /// Characters of Japanese and Chinese together, as stored.
    pub chars: u32,
    pub missing_ja: u32,
    pub missing_zh: u32,
}

/// An episode's lines in order, with the languages they have audio in.
pub async fn lines_of(db: &impl ConnectionTrait, episode_id: i64) -> Result<Vec<LineView>, DbErr> {
    let rows = line::Entity::find()
        .filter(line::Column::EpisodeId.eq(episode_id))
        .order_by_asc(line::Column::Id)
        .all(db)
        .await?;
    let mut audio = audio_of_episode(db, episode_id).await?;
    Ok(rows
        .into_iter()
        .map(|row| LineView {
            audio: audio.remove(&row.id).unwrap_or_default(),
            id: LineId(row.id),
            speaker: row.speaker,
            voice: row.voice,
            ja: row.ja,
            zh: row.zh,
        })
        .collect())
}

/// Languages each line of an episode has audio in, by line id. Reads no audio.
pub async fn audio_of_episode(
    db: &impl ConnectionTrait,
    episode_id: i64,
) -> Result<HashMap<i64, Vec<Language>>, DbErr> {
    let rows: Vec<(i64, String)> = line_audio::Entity::find()
        .select_only()
        .column(line_audio::Column::LineId)
        .column(line_audio::Column::Language)
        .join(JoinType::InnerJoin, line_audio::Relation::Line.def())
        .filter(line::Column::EpisodeId.eq(episode_id))
        .order_by_asc(line_audio::Column::Language)
        .into_tuple()
        .all(db)
        .await?;
    let mut found: HashMap<i64, Vec<Language>> = HashMap::new();
    for (id, code) in rows {
        found
            .entry(id)
            .or_default()
            .push(Language::from_code(&code)?);
    }
    Ok(found)
}

/// Each speaker of an episode with the voice it was mapped to, in order of
/// first appearance.
pub async fn speaker_voices(
    db: &impl ConnectionTrait,
    episode_id: i64,
) -> Result<Vec<(String, String)>, DbErr> {
    let rows: Vec<(String, String)> = line::Entity::find()
        .select_only()
        .column(line::Column::Speaker)
        .column(line::Column::Voice)
        .filter(line::Column::EpisodeId.eq(episode_id))
        .order_by_asc(line::Column::Id)
        .into_tuple()
        .all(db)
        .await?;
    let mut pairs: Vec<(String, String)> = Vec::new();
    for (speaker, voice) in rows {
        if !pairs.iter().any(|(seen, _)| *seen == speaker) {
            pairs.push((speaker, voice));
        }
    }
    Ok(pairs)
}

/// Line counts of every episode of a world that has lines, by episode id.
pub async fn line_counts(
    db: &impl ConnectionTrait,
    world_id: i64,
) -> Result<HashMap<i64, LineCounts>, DbErr> {
    let characters = Func::char_length(Expr::col((line::Entity, line::Column::Ja))).add(
        Func::char_length(Expr::col((line::Entity, line::Column::Zh))),
    );
    let lines: Vec<(i64, i64, i64)> = line::Entity::find()
        .select_only()
        .column(line::Column::EpisodeId)
        .column_as(Expr::col((line::Entity, line::Column::Id)).count(), "lines")
        .column_as(Expr::expr(Func::sum(characters)), "chars")
        .join(JoinType::InnerJoin, line::Relation::Episode.def())
        .filter(episode::Column::WorldId.eq(world_id))
        .group_by(line::Column::EpisodeId)
        .into_tuple()
        .all(db)
        .await?;
    let audio: Vec<(i64, String, i64)> = line_audio::Entity::find()
        .select_only()
        .column(line::Column::EpisodeId)
        .column(line_audio::Column::Language)
        .column_as(
            Expr::col((line_audio::Entity, line_audio::Column::LineId)).count(),
            "audio",
        )
        .join(JoinType::InnerJoin, line_audio::Relation::Line.def())
        .join(JoinType::InnerJoin, line::Relation::Episode.def())
        .filter(episode::Column::WorldId.eq(world_id))
        .group_by(line::Column::EpisodeId)
        .group_by(line_audio::Column::Language)
        .into_tuple()
        .all(db)
        .await?;

    let mut counts: HashMap<i64, LineCounts> = HashMap::new();
    for (episode_id, total, chars) in lines {
        let total = to_count(total);
        counts.insert(
            episode_id,
            LineCounts {
                lines: total,
                chars: to_count(chars),
                missing_ja: total,
                missing_zh: total,
            },
        );
    }
    for (episode_id, code, present) in audio {
        let Some(count) = counts.get_mut(&episode_id) else {
            continue;
        };
        let missing = match Language::from_code(&code)? {
            Language::Ja => &mut count.missing_ja,
            Language::Zh => &mut count.missing_zh,
        };
        *missing = missing.saturating_sub(to_count(present));
    }
    Ok(counts)
}

fn to_count(value: i64) -> u32 {
    u32::try_from(value).unwrap_or(0)
}
