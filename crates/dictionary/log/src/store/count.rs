use sea_orm::{ConnectionTrait, DbErr, EntityTrait, PaginatorTrait};

use crate::entity::log;

pub async fn count(db: &impl ConnectionTrait) -> Result<u32, DbErr> {
    let total = log::Entity::find().count(db).await?;
    Ok(u32::try_from(total).unwrap_or(u32::MAX))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::append::append;
    use crate::store::test_db::{memory_db, record};

    #[tokio::test]
    async fn counts_records() {
        let db = memory_db().await;
        assert_eq!(count(&db).await.unwrap(), 0);
        append(&db, &record(0)).await.unwrap();
        append(&db, &record(5)).await.unwrap();
        assert_eq!(count(&db).await.unwrap(), 2);
    }
}
