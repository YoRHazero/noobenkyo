use sea_orm::{ConnectionTrait, DbErr, EntityTrait, PaginatorTrait};

use crate::entity::log;

pub async fn count(db: &impl ConnectionTrait) -> Result<u32, DbErr> {
    let total = log::Entity::find().count(db).await?;
    Ok(u32::try_from(total).unwrap_or(u32::MAX))
}
