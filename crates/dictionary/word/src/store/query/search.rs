//! A page of rows for the dictionary page.

use std::collections::HashMap;

use sea_orm::{ConnectionTrait, DbErr};
use serde::Serialize;

use super::row_data::{RowData, load_row_data};
use super::{ContentFilter, Row, matching_rows};
use crate::WordKey;
use crate::store::tables::{label, sense as sense_table, sentence};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct SearchPage {
    pub rows: Vec<SearchRow>,
    /// Rows matching the filter, over all pages.
    pub total: u32,
}

/// One row, as an exported line would have it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct SearchRow {
    pub word: WordKey,
    /// `None` for a word with no readings.
    pub kana: Option<String>,
    /// `None` for a word or reading with nothing below it.
    pub sense: Option<SenseSummary>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct SenseSummary {
    pub primary_gloss: String,
    pub secondary_glosses: Vec<String>,
    pub labels: Vec<String>,
    pub stage: u32,
    pub passes: u32,
    pub lapses: u32,
    pub sentence_count: u32,
}

/// Rows matching the filter, `limit` of them starting at `offset`.
pub async fn search(
    db: &impl ConnectionTrait,
    filter: &ContentFilter,
    offset: u32,
    limit: u32,
) -> Result<SearchPage, DbErr> {
    let rows = matching_rows(db, filter).await?;
    let total = u32::try_from(rows.len()).unwrap_or(u32::MAX);
    let page: Vec<Row> = rows
        .into_iter()
        .skip(usize::try_from(offset).unwrap_or(usize::MAX))
        .take(usize::try_from(limit).unwrap_or(usize::MAX))
        .collect();
    let data = load_row_data(db, &page).await?;
    let sense_ids: Vec<i64> = page.iter().filter_map(|row| row.sense_id).collect();
    let mut labels: HashMap<i64, Vec<String>> = HashMap::new();
    for row in label::labels_of_senses(db, &sense_ids).await? {
        labels.entry(row.sense_id).or_default().push(row.text);
    }
    let counts = sentence::sentence_counts(db, &sense_ids).await?;
    let rows = page
        .iter()
        .filter_map(|row| search_row(row, &data, &mut labels, &counts))
        .collect();
    Ok(SearchPage { rows, total })
}

fn search_row(
    row: &Row,
    data: &RowData,
    labels: &mut HashMap<i64, Vec<String>>,
    counts: &HashMap<i64, u32>,
) -> Option<SearchRow> {
    let word = data.words.get(&row.word_id)?;
    let kana = match row.reading_id {
        Some(id) => Some(data.readings.get(&id)?.kana.clone()),
        None => None,
    };
    let sense = match row.sense_id {
        Some(id) => {
            let model = data.senses.get(&id)?;
            let progress = sense_table::progress(model);
            Some(SenseSummary {
                primary_gloss: model.primary_gloss.clone(),
                secondary_glosses: sense_table::secondary_glosses(model),
                labels: labels.remove(&id).unwrap_or_default(),
                stage: progress.stage,
                passes: progress.passes,
                lapses: progress.lapses,
                sentence_count: counts.get(&id).copied().unwrap_or(0),
            })
        }
        None => None,
    };
    Some(SearchRow {
        word: WordKey {
            writing: word.writing.clone(),
            origin: word.origin.clone(),
        },
        kana,
        sense,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::tables::label::add_labels;
    use crate::store::tables::sense::set_secondary_glosses;
    use crate::store::tables::sentence::insert_sentence;
    use crate::store::tables::word::insert_word;
    use crate::store::test_db::{memory_db, sense_of};

    #[tokio::test]
    async fn pages_rows_with_their_summaries() {
        let db = memory_db().await;
        let ashita = sense_of(&db, "明日", "あした", "明天").await;
        set_secondary_glosses(&db, ashita, &["翌日".to_owned()])
            .await
            .unwrap();
        add_labels(&db, ashita, &["N5".to_owned()]).await.unwrap();
        insert_sentence(&db, ashita, "[[{明日|あした}]]", None)
            .await
            .unwrap();
        insert_word(&db, "パソコン", "", "").await.unwrap();

        let page = search(&db, &ContentFilter::default(), 0, 10).await.unwrap();
        assert_eq!(page.total, 2);
        let sense = page.rows[0].sense.as_ref().unwrap();
        assert_eq!(page.rows[0].kana.as_deref(), Some("あした"));
        assert_eq!(sense.primary_gloss, "明天");
        assert_eq!(sense.secondary_glosses, ["翌日"]);
        assert_eq!(sense.labels, ["N5"]);
        assert_eq!(sense.sentence_count, 1);
        assert_eq!(
            page.rows[1],
            SearchRow {
                word: WordKey {
                    writing: "パソコン".into(),
                    origin: String::new()
                },
                kana: None,
                sense: None,
            }
        );

        let second = search(&db, &ContentFilter::default(), 1, 10).await.unwrap();
        assert_eq!(second.total, 2);
        assert_eq!(second.rows.len(), 1);
    }
}
