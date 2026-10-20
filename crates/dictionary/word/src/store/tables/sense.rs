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
