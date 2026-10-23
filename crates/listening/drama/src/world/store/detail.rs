use sea_orm::{ConnectionTrait, DbErr};

use super::cast::read::members;
use crate::by_key;
use crate::character::store::languages::languages_by_id;
use crate::{CastMember, WorldDetail};

/// One world with its cast; `None` if it is gone.
pub async fn world(db: &impl ConnectionTrait, name: &str) -> Result<Option<WorldDetail>, DbErr> {
    let Some(found) = by_key::find_world(db, name).await? else {
        return Ok(None);
    };
    let members = members(db, found.id).await?;
    let ids: Vec<i64> = members.iter().map(|member| member.character.id).collect();
    let mut languages = languages_by_id(db, Some(&ids)).await?;
    Ok(Some(WorldDetail {
        name: found.name,
        setting: found.setting,
        cast: members
            .into_iter()
            .map(|member| CastMember {
                languages: languages.remove(&member.character.id).unwrap_or_default(),
                character: member.character.name,
                role: member.role,
            })
            .collect(),
    }))
}
