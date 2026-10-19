//! Finding rows by natural key.

use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, JoinType, QueryFilter, QuerySelect,
    RelationTrait,
};

use crate::entity::{reading, sense, word};
use crate::store::tables::{reading as reading_table, word as word_table};
use crate::{ReadingKey, SenseKey, WordKey};

pub async fn find_word(
    db: &impl ConnectionTrait,
    key: &WordKey,
) -> Result<Option<word::Model>, DbErr> {
    word_table::find_word(db, &key.writing, &key.origin).await
}

pub async fn find_reading(
    db: &impl ConnectionTrait,
    key: &ReadingKey,
) -> Result<Option<reading::Model>, DbErr> {
    let Some(word) = word_table::find_word(db, &key.writing, &key.origin).await? else {
        return Ok(None);
    };
    reading_table::find_reading(db, word.id, &key.kana).await
}

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

    fn sense_key(kana: &str, gloss: &str) -> SenseKey {
        SenseKey {
            writing: "明日".into(),
            origin: String::new(),
            kana: kana.into(),
            primary_gloss: gloss.into(),
        }
    }

    #[tokio::test]
    async fn finds_each_layer_by_natural_key() {
        let db = memory_db().await;
        sense_of(&db, "明日", "あした", "明天").await;
        let asu = sense_of(&db, "明日", "あす", "明天").await;
        assert_eq!(
            find_sense(&db, &sense_key("あす", "明天"))
                .await
                .unwrap()
                .unwrap()
                .id,
            asu
        );
        assert!(
            find_sense(&db, &sense_key("あす", "将来"))
                .await
                .unwrap()
                .is_none()
        );
        let word = WordKey {
            writing: "明日".into(),
            origin: String::new(),
        };
        assert!(find_word(&db, &word).await.unwrap().is_some());
        let reading = ReadingKey {
            writing: "明日".into(),
            origin: String::new(),
            kana: "あした".into(),
        };
        assert!(find_reading(&db, &reading).await.unwrap().is_some());
        let missing = ReadingKey {
            kana: "みょうにち".into(),
            ..reading
        };
        assert!(find_reading(&db, &missing).await.unwrap().is_none());
    }
}
