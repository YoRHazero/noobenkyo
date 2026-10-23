//! Finding rows by the keys that leave drama: names, episode uuids and line
//! ids. Integer row ids never leave drama.

use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::entity::{character, episode, line, world};
use crate::{DramaError, LineId};

pub async fn find_character(
    db: &impl ConnectionTrait,
    name: &str,
) -> Result<Option<character::Model>, DbErr> {
    character::Entity::find()
        .filter(character::Column::Name.eq(name))
        .one(db)
        .await
}

pub async fn require_character(
    db: &impl ConnectionTrait,
    name: &str,
) -> Result<character::Model, DramaError> {
    find_character(db, name)
        .await?
        .ok_or_else(|| DramaError::CharacterNotFound(name.to_owned()))
}

pub async fn find_world(
    db: &impl ConnectionTrait,
    name: &str,
) -> Result<Option<world::Model>, DbErr> {
    world::Entity::find()
        .filter(world::Column::Name.eq(name))
        .one(db)
        .await
}

pub async fn require_world(
    db: &impl ConnectionTrait,
    name: &str,
) -> Result<world::Model, DramaError> {
    find_world(db, name)
        .await?
        .ok_or_else(|| DramaError::WorldNotFound(name.to_owned()))
}

pub async fn find_episode(
    db: &impl ConnectionTrait,
    uuid: Uuid,
) -> Result<Option<episode::Model>, DbErr> {
    episode::Entity::find()
        .filter(episode::Column::Uuid.eq(uuid))
        .one(db)
        .await
}

pub async fn require_episode(
    db: &impl ConnectionTrait,
    uuid: Uuid,
) -> Result<episode::Model, DramaError> {
    find_episode(db, uuid)
        .await?
        .ok_or(DramaError::EpisodeNotFound)
}

pub async fn find_line(
    db: &impl ConnectionTrait,
    id: LineId,
) -> Result<Option<line::Model>, DbErr> {
    line::Entity::find_by_id(id.row()).one(db).await
}

pub async fn require_line(
    db: &impl ConnectionTrait,
    id: LineId,
) -> Result<line::Model, DramaError> {
    find_line(db, id).await?.ok_or(DramaError::LineNotFound)
}
