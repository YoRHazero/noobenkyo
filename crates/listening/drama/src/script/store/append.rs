use std::collections::HashMap;

use rand::Rng;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set, TransactionTrait};
use uuid::Uuid;

use super::voices::voices_of;
use crate::by_key;
use crate::entity::{episode, world};
use crate::line::store::insert::insert_line;
use crate::line::store::of_episode::speaker_voices;
use crate::script::rules::voice_mapping::map_voices;
use crate::world::store::cast::read::complete_members;
use crate::{DramaError, Script};

/// Continues an episode: appends the script's lines and replaces the
/// summary with the script's, which covers the whole episode; the title is
/// not used. Speakers the episode already has keep their voice; new ones are
/// mapped with the world's current cast, which must be complete.
pub async fn append_script(
    db: &DatabaseConnection,
    uuid: Uuid,
    script: &Script,
    rng: &mut impl Rng,
) -> Result<(), DramaError> {
    let txn = db.begin().await?;
    let found = by_key::require_episode(&txn, uuid).await?;
    let owner = world::Entity::find_by_id(found.world_id)
        .one(&txn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("episode world".into()))?;
    let members = complete_members(&txn, &owner).await?;
    let mut mapping: HashMap<String, String> =
        speaker_voices(&txn, found.id).await?.into_iter().collect();
    map_voices(
        &voices_of(&members),
        &mut mapping,
        script.lines.iter().map(|line| line.speaker.as_str()),
        rng,
    );
    for line in &script.lines {
        let voice = &mapping[&line.speaker];
        insert_line(&txn, found.id, &line.speaker, voice, &line.ja, &line.zh).await?;
    }
    episode::ActiveModel {
        id: Set(found.id),
        summary: Set(script.summary.clone()),
        ..Default::default()
    }
    .update(&txn)
    .await?;
    txn.commit().await?;
    Ok(())
}
