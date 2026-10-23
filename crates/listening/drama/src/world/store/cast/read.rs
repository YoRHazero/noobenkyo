use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder};

use crate::entity::{cast, character, world};
use crate::world::rules::cast_complete::missing_roles;
use crate::{DramaError, Role};

/// A character in a world's cast.
#[derive(Clone, Debug)]
pub struct Member {
    pub character: character::Model,
    pub role: Role,
}

/// A world's cast: the narrator first, then main characters, then minor
/// ones, each in the order the characters were created.
pub async fn members(db: &impl ConnectionTrait, world_id: i64) -> Result<Vec<Member>, DbErr> {
    let rows = cast::Entity::find()
        .filter(cast::Column::WorldId.eq(world_id))
        .find_also_related(character::Entity)
        .order_by_asc(cast::Column::CharacterId)
        .all(db)
        .await?;
    let mut members = Vec::with_capacity(rows.len());
    for (row, found) in rows {
        let character = found.ok_or_else(|| DbErr::RecordNotFound("cast character".into()))?;
        members.push(Member {
            character,
            role: Role::from_columns(&row.role, row.description)?,
        });
    }
    members.sort_by_key(|member| match member.role {
        Role::Narrator => 0,
        Role::Main { .. } => 1,
        Role::Minor => 2,
    });
    Ok(members)
}

/// A world's cast, when it has the narrator and main characters a script needs.
pub async fn complete_members(
    db: &impl ConnectionTrait,
    world: &world::Model,
) -> Result<Vec<Member>, DramaError> {
    let members = members(db, world.id).await?;
    let has_narrator = members.iter().any(|member| member.role == Role::Narrator);
    let mains = members
        .iter()
        .filter(|member| matches!(member.role, Role::Main { .. }))
        .count();
    match missing_roles(has_narrator, mains) {
        Some(missing) => Err(DramaError::CastIncomplete {
            world: world.name.clone(),
            missing,
        }),
        None => Ok(members),
    }
}
