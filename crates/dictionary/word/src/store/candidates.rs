use sea_orm::sea_query::Query;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, FromQueryResult, JoinType, QueryFilter,
    QueryOrder, QuerySelect, RelationTrait,
};

use crate::entity::{reading, sense, sentence, word};
use crate::{Progress, SenseCandidate, SenseKey};

#[derive(FromQueryResult)]
struct CandidateRow {
    writing: String,
    origin: String,
    kana: String,
    primary_gloss: String,
    stage: i64,
    passes: i64,
    lapses: i64,
    modified_at: chrono::DateTime<chrono::FixedOffset>,
}

/// Senses that have at least one sentence, in creation (rowid) order.
pub async fn list_candidates(db: &impl ConnectionTrait) -> Result<Vec<SenseCandidate>, DbErr> {
    let rows = sense::Entity::find()
        .select_only()
        .column(word::Column::Writing)
        .column(word::Column::Origin)
        .column(reading::Column::Kana)
        .column(sense::Column::PrimaryGloss)
        .column(sense::Column::Stage)
        .column(sense::Column::Passes)
        .column(sense::Column::Lapses)
        .column(sense::Column::ModifiedAt)
        .join(JoinType::InnerJoin, sense::Relation::Reading.def())
        .join(JoinType::InnerJoin, reading::Relation::Word.def())
        .filter(
            sense::Column::Id.in_subquery(
                Query::select()
                    .column(sentence::Column::SenseId)
                    .from(sentence::Entity)
                    .to_owned(),
            ),
        )
        .order_by_asc(sense::Column::Id)
        .into_model::<CandidateRow>()
        .all(db)
        .await?;
    Ok(rows.into_iter().map(to_candidate).collect())
}

fn to_candidate(row: CandidateRow) -> SenseCandidate {
    SenseCandidate {
        key: SenseKey {
            writing: row.writing,
            origin: row.origin,
            kana: row.kana,
            primary_gloss: row.primary_gloss,
        },
        progress: Progress {
            stage: u32::try_from(row.stage).unwrap_or(0),
            passes: u32::try_from(row.passes).unwrap_or(0),
            lapses: u32::try_from(row.lapses).unwrap_or(0),
            modified_at: row.modified_at,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::sentence::insert_sentence;
    use crate::store::test_db::{memory_db, progress_at, sense_of};

    #[tokio::test]
    async fn lists_only_senses_with_sentences_in_creation_order() {
        let db = memory_db().await;
        let later = sense_of(&db, "諦める", "あきらめる", "放弃").await;
        sense_of(&db, "明日", "あした", "明天").await;
        let first_with_sentence = sense_of(&db, "明日", "あす", "明天").await;
        insert_sentence(&db, first_with_sentence, "[[{明日|あす}]]", None)
            .await
            .unwrap();
        insert_sentence(&db, later, "[[{諦|あきら}める]]", None)
            .await
            .unwrap();
        insert_sentence(&db, later, "[[{諦|あきら}めた]]", None)
            .await
            .unwrap();

        let candidates = list_candidates(&db).await.unwrap();
        let kana: Vec<_> = candidates.iter().map(|c| c.key.kana.as_str()).collect();
        assert_eq!(kana, ["あきらめる", "あす"]);
        assert_eq!(candidates[0].progress, progress_at(0, 0, 0));
        assert_eq!(candidates[0].key.primary_gloss, "放弃");
    }
}
