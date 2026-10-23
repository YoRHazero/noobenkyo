use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, Set};

use crate::entity::line;

/// Appends a line to an episode, without audio.
pub async fn insert_line(
    db: &impl ConnectionTrait,
    episode_id: i64,
    speaker: &str,
    voice: &str,
    ja: &str,
    zh: &str,
) -> Result<(), DbErr> {
    line::ActiveModel {
        episode_id: Set(episode_id),
        speaker: Set(speaker.to_owned()),
        voice: Set(voice.to_owned()),
        ja: Set(ja.to_owned()),
        zh: Set(zh.to_owned()),
        ..Default::default()
    }
    .insert(db)
    .await
    .map(|_| ())
}
