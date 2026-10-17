use sea_orm::{ConnectionTrait, DbErr, EntityTrait};

use crate::entity::log;

/// Deletes every record.
pub async fn clear(db: &impl ConnectionTrait) -> Result<(), DbErr> {
    log::Entity::delete_many().exec(db).await.map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::append::append;
    use crate::store::count::count;
    use crate::store::test_db::{memory_db, record};

    #[tokio::test]
    async fn removes_all_records() {
        let db = memory_db().await;
        append(&db, &record(0)).await.unwrap();
        clear(&db).await.unwrap();
        assert_eq!(count(&db).await.unwrap(), 0);
    }
}
