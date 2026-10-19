use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    Set,
};

use super::ID_CHUNK;
use crate::Progress;
use crate::entity::sense;

/// Secondary glosses stored in the JSON column.
pub fn secondary_glosses(model: &sense::Model) -> Vec<String> {
    serde_json::from_value(model.secondary_glosses.clone()).unwrap_or_default()
}

/// All glosses of a sense, the primary gloss first.
pub fn glosses(model: &sense::Model) -> Vec<String> {
    let mut all = vec![model.primary_gloss.clone()];
    all.extend(secondary_glosses(model));
    all
}

pub fn progress(model: &sense::Model) -> Progress {
    Progress {
        stage: to_count(model.stage),
        passes: to_count(model.passes),
        lapses: to_count(model.lapses),
        modified_at: model.modified_at,
    }
}

fn to_count(value: i64) -> u32 {
    u32::try_from(value).unwrap_or(0)
}

pub async fn senses_of_reading(
    db: &impl ConnectionTrait,
    reading_id: i64,
) -> Result<Vec<sense::Model>, DbErr> {
    sense::Entity::find()
        .filter(sense::Column::ReadingId.eq(reading_id))
        .order_by_asc(sense::Column::Id)
        .all(db)
        .await
}

pub async fn insert_sense(
    db: &impl ConnectionTrait,
    reading_id: i64,
    primary_gloss: &str,
    secondary_glosses: &[String],
    progress: &Progress,
) -> Result<sense::Model, DbErr> {
    sense::ActiveModel {
        reading_id: Set(reading_id),
        primary_gloss: Set(primary_gloss.to_owned()),
        secondary_glosses: Set(serde_json::json!(secondary_glosses)),
        stage: Set(progress.stage.into()),
        passes: Set(progress.passes.into()),
        lapses: Set(progress.lapses.into()),
        modified_at: Set(progress.modified_at),
        ..Default::default()
    }
    .insert(db)
    .await
}

/// Senses with the given ids, in id order.
pub async fn senses_by_ids(
    db: &impl ConnectionTrait,
    ids: &[i64],
) -> Result<Vec<sense::Model>, DbErr> {
    let mut found = Vec::with_capacity(ids.len());
    for chunk in ids.chunks(ID_CHUNK) {
        let rows = sense::Entity::find()
            .filter(sense::Column::Id.is_in(chunk.iter().copied()))
            .order_by_asc(sense::Column::Id)
            .all(db)
            .await?;
        found.extend(rows);
    }
    Ok(found)
}

/// Replaces all glosses of a sense, the primary one included.
pub async fn set_glosses(
    db: &impl ConnectionTrait,
    id: i64,
    primary_gloss: &str,
    secondary_glosses: &[String],
) -> Result<(), DbErr> {
    sense::ActiveModel {
        id: Set(id),
        primary_gloss: Set(primary_gloss.to_owned()),
        secondary_glosses: Set(serde_json::json!(secondary_glosses)),
        ..Default::default()
    }
    .update(db)
    .await
    .map(|_| ())
}

pub async fn set_secondary_glosses(
    db: &impl ConnectionTrait,
    id: i64,
    secondary_glosses: &[String],
) -> Result<(), DbErr> {
    sense::ActiveModel {
        id: Set(id),
        secondary_glosses: Set(serde_json::json!(secondary_glosses)),
        ..Default::default()
    }
    .update(db)
    .await
    .map(|_| ())
}

pub async fn set_progress(
    db: &impl ConnectionTrait,
    id: i64,
    progress: &Progress,
) -> Result<(), DbErr> {
    sense::ActiveModel {
        id: Set(id),
        stage: Set(progress.stage.into()),
        passes: Set(progress.passes.into()),
        lapses: Set(progress.lapses.into()),
        modified_at: Set(progress.modified_at),
        ..Default::default()
    }
    .update(db)
    .await
    .map(|_| ())
}

pub async fn delete_sense(db: &impl ConnectionTrait, id: i64) -> Result<(), DbErr> {
    sense::Entity::delete_by_id(id).exec(db).await.map(|_| ())
}

/// All senses in creation order.
#[cfg(test)]
pub async fn all_senses(db: &impl ConnectionTrait) -> Result<Vec<sense::Model>, DbErr> {
    sense::Entity::find()
        .order_by_asc(sense::Column::Id)
        .all(db)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::test_db::{memory_db, progress_at, reading_of};

    #[tokio::test]
    async fn inserts_and_lists_senses_of_reading() {
        let db = memory_db().await;
        let reading = reading_of(&db, "明日", "あした").await;
        let progress = progress_at(1, 2, 3);
        insert_sense(&db, reading, "明天", &["翌日".to_owned()], &progress)
            .await
            .unwrap();
        let senses = senses_of_reading(&db, reading).await.unwrap();
        assert_eq!(senses.len(), 1);
        assert_eq!(glosses(&senses[0]), ["明天", "翌日"]);
        assert_eq!(super::progress(&senses[0]), progress);
    }

    #[tokio::test]
    async fn rejects_duplicate_primary_gloss() {
        let db = memory_db().await;
        let reading = reading_of(&db, "明日", "あした").await;
        insert_sense(&db, reading, "明天", &[], &progress_at(0, 0, 0))
            .await
            .unwrap();
        assert!(
            insert_sense(&db, reading, "明天", &[], &progress_at(0, 0, 0))
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn updates_glosses_and_progress() {
        let db = memory_db().await;
        let reading = reading_of(&db, "明日", "あした").await;
        let sense = insert_sense(&db, reading, "明天", &[], &progress_at(0, 0, 0))
            .await
            .unwrap();
        set_secondary_glosses(&db, sense.id, &["翌日".to_owned()])
            .await
            .unwrap();
        set_progress(&db, sense.id, &progress_at(4, 5, 6))
            .await
            .unwrap();
        let stored = &all_senses(&db).await.unwrap()[0];
        assert_eq!(secondary_glosses(stored), ["翌日"]);
        assert_eq!(super::progress(stored), progress_at(4, 5, 6));
    }

    #[tokio::test]
    async fn deletes_sense() {
        let db = memory_db().await;
        let reading = reading_of(&db, "明日", "あした").await;
        let sense = insert_sense(&db, reading, "明天", &[], &progress_at(0, 0, 0))
            .await
            .unwrap();
        delete_sense(&db, sense.id).await.unwrap();
        assert!(all_senses(&db).await.unwrap().is_empty());
    }
}
