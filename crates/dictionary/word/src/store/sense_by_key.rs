use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, JoinType, QueryFilter, QuerySelect,
    RelationTrait,
};

use crate::SenseKey;
use crate::entity::{reading, sense, word};

/// Finds a sense by its natural key.
pub async fn find_sense(
    db: &impl ConnectionTrait,
    key: &SenseKey,
) -> Result<Option<sense::Model>, DbErr> {
    sense::Entity::find()
        .join(JoinType::InnerJoin, sense::Relation::Reading.def())
        .join(JoinType::InnerJoin, reading::Relation::Word.def())
        .filter(word::Column::Writing.eq(&key.writing))
        .filter(word::Column::Origin.eq(&key.origin))
        .filter(reading::Column::Kana.eq(&key.kana))
        .filter(sense::Column::PrimaryGloss.eq(&key.primary_gloss))
        .one(db)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::test_db::{memory_db, sense_of};

    fn key(kana: &str, gloss: &str) -> SenseKey {
        SenseKey {
            writing: "明日".into(),
            origin: String::new(),
            kana: kana.into(),
            primary_gloss: gloss.into(),
        }
    }

    #[tokio::test]
    async fn finds_by_natural_key() {
        let db = memory_db().await;
        sense_of(&db, "明日", "あした", "明天").await;
        let asu = sense_of(&db, "明日", "あす", "明天").await;
        assert_eq!(
            find_sense(&db, &key("あす", "明天"))
                .await
                .unwrap()
                .unwrap()
                .id,
            asu
        );
        assert!(
            find_sense(&db, &key("あす", "将来"))
                .await
                .unwrap()
                .is_none()
        );
    }
}
