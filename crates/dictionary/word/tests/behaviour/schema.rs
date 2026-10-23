//! Constraints of the tables, as the migration sets them up.

use dictionary_word::entity::{label, reading, sense, word};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};

use crate::support::{memory_db, now};

async fn insert_word(
    db: &DatabaseConnection,
    writing: &str,
    origin: &str,
) -> Result<word::Model, DbErr> {
    word::ActiveModel {
        writing: Set(writing.into()),
        origin: Set(origin.into()),
        origin_language: Set(String::new()),
        ..Default::default()
    }
    .insert(db)
    .await
}

async fn insert_reading(
    db: &DatabaseConnection,
    word_id: i64,
    kana: &str,
) -> Result<reading::Model, DbErr> {
    reading::ActiveModel {
        word_id: Set(word_id),
        kana: Set(kana.into()),
        ..Default::default()
    }
    .insert(db)
    .await
}

async fn insert_sense(
    db: &DatabaseConnection,
    reading_id: i64,
    gloss: &str,
) -> Result<sense::Model, DbErr> {
    sense::ActiveModel {
        reading_id: Set(reading_id),
        primary_gloss: Set(gloss.into()),
        secondary_glosses: Set(serde_json::json!([])),
        stage: Set(0),
        passes: Set(0),
        lapses: Set(0),
        modified_at: Set(now()),
        ..Default::default()
    }
    .insert(db)
    .await
}

#[tokio::test]
async fn a_word_is_unique_by_writing_and_origin() {
    let db = memory_db().await;
    insert_word(&db, "コート", "coat").await.unwrap();
    insert_word(&db, "コート", "court").await.unwrap();
    assert!(insert_word(&db, "コート", "coat").await.is_err());
}

#[tokio::test]
async fn a_reading_is_unique_within_its_word() {
    let db = memory_db().await;
    let word = insert_word(&db, "明日", "").await.unwrap();
    insert_reading(&db, word.id, "あした").await.unwrap();
    insert_reading(&db, word.id, "あす").await.unwrap();
    assert!(insert_reading(&db, word.id, "あす").await.is_err());
}

#[tokio::test]
async fn a_primary_gloss_is_unique_within_its_reading() {
    let db = memory_db().await;
    let word = insert_word(&db, "明日", "").await.unwrap();
    let ashita = insert_reading(&db, word.id, "あした").await.unwrap();
    let asu = insert_reading(&db, word.id, "あす").await.unwrap();
    insert_sense(&db, ashita.id, "明天").await.unwrap();
    insert_sense(&db, asu.id, "明天").await.unwrap();
    assert!(insert_sense(&db, ashita.id, "明天").await.is_err());
}

#[tokio::test]
async fn a_label_is_unique_within_its_sense() {
    let db = memory_db().await;
    let word = insert_word(&db, "明日", "").await.unwrap();
    let reading = insert_reading(&db, word.id, "あした").await.unwrap();
    let sense = insert_sense(&db, reading.id, "明天").await.unwrap();
    let label = |text: &str| label::ActiveModel {
        sense_id: Set(sense.id),
        text: Set(text.into()),
    };
    label("N5").insert(&db).await.unwrap();
    label("时间").insert(&db).await.unwrap();
    assert!(label("N5").insert(&db).await.is_err());
}
