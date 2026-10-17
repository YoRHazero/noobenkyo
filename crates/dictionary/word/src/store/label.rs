use sea_orm::sea_query::OnConflict;
use sea_orm::{ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, Set};

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

/// All labels, grouped by sense in creation order.
pub async fn all_labels(db: &impl ConnectionTrait) -> Result<Vec<label::Model>, DbErr> {
    label::Entity::find()
        .order_by_asc(label::Column::SenseId)
        .order_by_asc(label::Column::Text)
        .all(db)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::test_db::{memory_db, sense_of};

    #[tokio::test]
    async fn adds_labels_as_a_union() {
        let db = memory_db().await;
        let sense = sense_of(&db, "明日", "あした", "明天").await;
        add_labels(&db, sense, &["N5".to_owned(), "时间".to_owned()])
            .await
            .unwrap();
        add_labels(&db, sense, &["N5".to_owned(), "名词".to_owned()])
            .await
            .unwrap();
        assert_eq!(
            labels_of_sense(&db, sense).await.unwrap(),
            ["N5", "名词", "时间"]
        );
        assert_eq!(all_labels(&db).await.unwrap().len(), 3);
    }
}
