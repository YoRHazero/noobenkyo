use sea_orm::sea_query::{Expr, ExprTrait};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, FromQueryResult, JoinType,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Select,
};
use serde::Serialize;

use crate::SenseKey;
use crate::entity::{reading, sense, sentence, word};
use crate::store::label;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct SenseSummary {
    pub key: SenseKey,
    pub secondary_glosses: Vec<String>,
    pub labels: Vec<String>,
    pub stage: u32,
    pub passes: u32,
    pub lapses: u32,
    pub sentence_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct SensePage {
    pub rows: Vec<SenseSummary>,
    pub total: u32,
}

#[derive(FromQueryResult)]
struct SummaryRow {
    id: i64,
    writing: String,
    origin: String,
    kana: String,
    primary_gloss: String,
    secondary_glosses: serde_json::Value,
    stage: i64,
    passes: i64,
    lapses: i64,
}

/// Senses whose writing, kana or glosses contain `query`, in creation order.
pub async fn search_senses(
    db: &impl ConnectionTrait,
    query: &str,
    offset: u32,
    limit: u32,
) -> Result<SensePage, DbErr> {
    let total = matching(query).count(db).await?;
    let rows = matching(query)
        .select_only()
        .column(sense::Column::Id)
        .column(word::Column::Writing)
        .column(word::Column::Origin)
        .column(reading::Column::Kana)
        .column(sense::Column::PrimaryGloss)
        .column(sense::Column::SecondaryGlosses)
        .column(sense::Column::Stage)
        .column(sense::Column::Passes)
        .column(sense::Column::Lapses)
        .order_by_asc(sense::Column::Id)
        .offset(u64::from(offset))
        .limit(u64::from(limit))
        .into_model::<SummaryRow>()
        .all(db)
        .await?;
    let mut summaries = Vec::with_capacity(rows.len());
    for row in rows {
        summaries.push(summarize(db, row).await?);
    }
    Ok(SensePage {
        rows: summaries,
        total: u32::try_from(total).unwrap_or(u32::MAX),
    })
}

fn matching(query: &str) -> Select<sense::Entity> {
    let select = sense::Entity::find()
        .join(JoinType::InnerJoin, sense::Relation::Reading.def())
        .join(JoinType::InnerJoin, reading::Relation::Word.def());
    let query = query.trim();
    if query.is_empty() {
        return select;
    }
    select.filter(
        Condition::any()
            .add(word::Column::Writing.contains(query))
            .add(reading::Column::Kana.contains(query))
            .add(sense::Column::PrimaryGloss.contains(query))
            .add(
                Expr::col((sense::Entity, sense::Column::SecondaryGlosses))
                    .like(format!("%{query}%")),
            ),
    )
}

async fn summarize(db: &impl ConnectionTrait, row: SummaryRow) -> Result<SenseSummary, DbErr> {
    let labels = label::labels_of_sense(db, row.id).await?;
    let sentence_count = sentence::Entity::find()
        .filter(sentence::Column::SenseId.eq(row.id))
        .count(db)
        .await?;
    Ok(SenseSummary {
        key: SenseKey {
            writing: row.writing,
            origin: row.origin,
            kana: row.kana,
            primary_gloss: row.primary_gloss,
        },
        secondary_glosses: serde_json::from_value(row.secondary_glosses).unwrap_or_default(),
        labels,
        stage: u32::try_from(row.stage).unwrap_or(0),
        passes: u32::try_from(row.passes).unwrap_or(0),
        lapses: u32::try_from(row.lapses).unwrap_or(0),
        sentence_count: u32::try_from(sentence_count).unwrap_or(u32::MAX),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::label::add_labels;
    use crate::store::sense::set_secondary_glosses;
    use crate::store::sentence::insert_sentence;
    use crate::store::test_db::{memory_db, sense_of};

    #[tokio::test]
    async fn searches_writing_kana_and_glosses() {
        let db = memory_db().await;
        let ashita = sense_of(&db, "明日", "あした", "明天").await;
        sense_of(&db, "諦める", "あきらめる", "放弃").await;
        set_secondary_glosses(&db, ashita, &["翌日".to_owned()])
            .await
            .unwrap();
        add_labels(&db, ashita, &["N5".to_owned()]).await.unwrap();
        insert_sentence(&db, ashita, "[[{明日|あした}]]", None)
            .await
            .unwrap();

        for query in ["明日", "あした", "明天", "翌日"] {
            let page = search_senses(&db, query, 0, 10).await.unwrap();
            assert_eq!(page.total, 1, "query {query}");
            assert_eq!(page.rows[0].key.kana, "あした");
        }
        let row = &search_senses(&db, "翌日", 0, 10).await.unwrap().rows[0];
        assert_eq!(row.labels, ["N5"]);
        assert_eq!(row.sentence_count, 1);
        assert_eq!(row.secondary_glosses, ["翌日"]);
    }

    #[tokio::test]
    async fn pages_all_senses_for_empty_query() {
        let db = memory_db().await;
        sense_of(&db, "明日", "あした", "明天").await;
        sense_of(&db, "諦める", "あきらめる", "放弃").await;
        let page = search_senses(&db, "", 1, 10).await.unwrap();
        assert_eq!(page.total, 2);
        assert_eq!(page.rows.len(), 1);
        assert_eq!(page.rows[0].key.writing, "諦める");
    }
}
