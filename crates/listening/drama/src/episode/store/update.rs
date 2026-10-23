use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

use crate::DramaError;
use crate::by_key;
use crate::entity::episode;
use crate::episode::rules::fields::check_fields;

/// Replaces an episode's number, title and summary together. Returns the
/// problems that rejected them, empty when they were replaced.
pub async fn update_episode(
    db: &impl ConnectionTrait,
    uuid: Uuid,
    number: f64,
    title: &str,
    summary: &str,
) -> Result<Vec<String>, DramaError> {
    let (number, title, summary) = match check_fields(number, title, summary) {
        Ok(fields) => fields,
        Err(problems) => return Ok(problems),
    };
    let found = by_key::require_episode(db, uuid).await?;
    episode::ActiveModel {
        id: Set(found.id),
        number: Set(number),
        title: Set(title),
        summary: Set(summary),
        ..Default::default()
    }
    .update(db)
    .await?;
    Ok(Vec::new())
}
