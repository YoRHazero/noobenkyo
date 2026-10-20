use sea_orm::sea_query::OnConflict;
use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, Set};

use super::ID_CHUNK;
use crate::entity::label;

/// Adds labels to a sense; labels it already has are ignored.
pub async fn add_labels(
    db: &impl ConnectionTrait,
    sense_id: i64,
    labels: &[String],
) -> Result<(), DbErr> {
    if labels.is_empty() {
        return Ok(());
    }
    let rows = labels.iter().map(|text| label::ActiveModel {
        sense_id: Set(sense_id),
        text: Set(text.clone()),
    });
    label::Entity::insert_many(rows)
        .on_conflict(
            OnConflict::columns([label::Column::SenseId, label::Column::Text])
                .do_nothing()
                .to_owned(),
        )
        .exec_without_returning(db)
        .await
        .map(|_| ())
}

pub async fn labels_of_sense(
    db: &impl ConnectionTrait,
    sense_id: i64,
) -> Result<Vec<String>, DbErr> {
    let rows = label::Entity::find()
        .filter(label::Column::SenseId.eq(sense_id))
        .order_by_asc(label::Column::Text)
        .all(db)
        .await?;
    Ok(rows.into_iter().map(|row| row.text).collect())
}

/// Labels of the given senses, by sense and then text.
pub async fn labels_of_senses(
    db: &impl ConnectionTrait,
    sense_ids: &[i64],
) -> Result<Vec<label::Model>, DbErr> {
    let mut found = Vec::new();
    for chunk in sense_ids.chunks(ID_CHUNK) {
        let rows = label::Entity::find()
            .filter(label::Column::SenseId.is_in(chunk.iter().copied()))
            .order_by_asc(label::Column::SenseId)
            .order_by_asc(label::Column::Text)
            .all(db)
            .await?;
        found.extend(rows);
    }
    Ok(found)
}

/// Replaces all labels of a sense.
pub async fn replace_labels(
    db: &impl ConnectionTrait,
    sense_id: i64,
    labels: &[String],
) -> Result<(), DbErr> {
    label::Entity::delete_many()
        .filter(label::Column::SenseId.eq(sense_id))
        .exec(db)
        .await?;
    add_labels(db, sense_id, labels).await
}
