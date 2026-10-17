use sea_orm::sea_query::OnConflict;
use sea_orm::{ConnectionTrait, DbErr, EntityTrait};

use crate::LogRecord;
use crate::entity::log;
use crate::rules::row::to_row;

/// Inserts a record; returns `false` if a record with the same key exists.
pub async fn append(db: &impl ConnectionTrait, record: &LogRecord) -> Result<bool, DbErr> {
    let inserted = log::Entity::insert(to_row(record))
        .on_conflict(
            OnConflict::columns([
                log::Column::Time,
                log::Column::Writing,
                log::Column::Origin,
                log::Column::Kana,
                log::Column::PrimaryGloss,
                log::Column::QuestionType,
            ])
            .do_nothing()
            .to_owned(),
        )
        .exec_without_returning(db)
        .await?;
    Ok(inserted > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::count::count;
    use crate::store::test_db::{memory_db, record};

    #[tokio::test]
    async fn appends_once_per_key() {
        let db = memory_db().await;
        assert!(append(&db, &record(0)).await.unwrap());
        assert!(!append(&db, &record(0)).await.unwrap());
        assert!(append(&db, &record(1)).await.unwrap());
        assert_eq!(count(&db).await.unwrap(), 2);
    }
}
