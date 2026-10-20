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
