use sea_orm::ConnectionTrait;
use uuid::Uuid;

use crate::by_key;
use crate::entity::world;
use crate::line::store::of_episode::lines_of;
use crate::world::store::cast::read::complete_members;
use crate::{DramaError, Gender, MainCharacter, Role, ScriptContext, ScriptEpisode, ScriptLine};

/// Everything a script for the world is written from: its setting, its main
/// characters, and the full text of the chosen predecessors (in the given
/// order) and of the episode being continued. The cast must have a narrator
/// and a main character, and the episodes must belong to the world.
pub async fn script_context(
    db: &impl ConnectionTrait,
    world: &str,
    predecessors: &[Uuid],
    appending: Option<Uuid>,
) -> Result<ScriptContext, DramaError> {
    let found = by_key::require_world(db, world).await?;
    let members = complete_members(db, &found).await?;
    let mut mains = Vec::new();
    for member in members {
        if let Role::Main { description } = member.role {
            mains.push(MainCharacter {
                gender: Gender::from_code(&member.character.gender)?,
                age: u32::try_from(member.character.age).unwrap_or(0),
                name: member.character.name,
                kana: member.character.kana,
                description,
            });
        }
    }
    let mut chosen = Vec::with_capacity(predecessors.len());
    for uuid in predecessors {
        chosen.push(episode_text(db, &found, *uuid).await?);
    }
    let appending = match appending {
        Some(uuid) => Some(episode_text(db, &found, uuid).await?),
        None => None,
    };
    Ok(ScriptContext {
        world: found.name,
        setting: found.setting,
        mains,
        predecessors: chosen,
        appending,
    })
}

async fn episode_text(
    db: &impl ConnectionTrait,
    world: &world::Model,
    uuid: Uuid,
) -> Result<ScriptEpisode, DramaError> {
    let found = by_key::require_episode(db, uuid).await?;
    if found.world_id != world.id {
        return Err(DramaError::ForeignEpisode(found.title, world.name.clone()));
    }
    let lines = lines_of(db, found.id)
        .await?
        .into_iter()
        .map(|line| ScriptLine {
            speaker: line.speaker,
            ja: line.ja,
            zh: line.zh,
        })
        .collect();
    Ok(ScriptEpisode {
        number: found.number,
        title: found.title,
        summary: found.summary,
        lines,
    })
}
