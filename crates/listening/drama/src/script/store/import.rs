use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use rand::Rng;
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

use super::voices::voices_of;
use crate::by_key;
use crate::episode::store::create::insert_episode;
use crate::line::store::insert::insert_line;
use crate::script::rules::voice_mapping::map_voices;
use crate::world::store::cast::read::complete_members;
use crate::{DramaError, Script};

/// Stores a script as a new episode of the world, numbered after its
/// largest number, with each speaker mapped to a voice of the current cast.
/// The cast must have a narrator and a main character.
pub async fn import_script(
    db: &DatabaseConnection,
    world: &str,
    script: &Script,
    uuid: Uuid,
    created_at: DateTime<FixedOffset>,
    rng: &mut impl Rng,
) -> Result<(), DramaError> {
    let txn = db.begin().await?;
    let found = by_key::require_world(&txn, world).await?;
    let members = complete_members(&txn, &found).await?;
    let mut mapping = HashMap::new();
    map_voices(
        &voices_of(&members),
        &mut mapping,
        script.lines.iter().map(|line| line.speaker.as_str()),
        rng,
    );
    let episode = insert_episode(
        &txn,
        found.id,
        uuid,
        &script.title,
        &script.summary,
        created_at,
    )
    .await?;
    for line in &script.lines {
        let voice = &mapping[&line.speaker];
        insert_line(&txn, episode.id, &line.speaker, voice, &line.ja, &line.zh).await?;
    }
    txn.commit().await?;
    Ok(())
}
