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
