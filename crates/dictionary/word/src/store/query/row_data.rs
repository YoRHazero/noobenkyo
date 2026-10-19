//! The word, reading and sense rows behind a list of rows, loaded at once.

use std::collections::HashMap;

use sea_orm::{ConnectionTrait, DbErr};

use super::Row;
use crate::entity::{reading, sense, word};
use crate::store::tables::{reading as reading_table, sense as sense_table, word as word_table};

pub struct RowData {
    pub words: HashMap<i64, word::Model>,
    pub readings: HashMap<i64, reading::Model>,
    pub senses: HashMap<i64, sense::Model>,
}

pub async fn load_row_data(db: &impl ConnectionTrait, rows: &[Row]) -> Result<RowData, DbErr> {
    let word_ids: Vec<i64> = rows.iter().map(|row| row.word_id).collect();
    let reading_ids: Vec<i64> = rows.iter().filter_map(|row| row.reading_id).collect();
    let sense_ids: Vec<i64> = rows.iter().filter_map(|row| row.sense_id).collect();
    Ok(RowData {
        words: by_id(word_table::words_by_ids(db, &dedup(word_ids)).await?, |w| {
            w.id
        }),
        readings: by_id(
            reading_table::readings_by_ids(db, &dedup(reading_ids)).await?,
            |r| r.id,
        ),
        senses: by_id(sense_table::senses_by_ids(db, &sense_ids).await?, |s| s.id),
    })
}

fn dedup(mut ids: Vec<i64>) -> Vec<i64> {
    ids.sort_unstable();
    ids.dedup();
    ids
}

fn by_id<T>(rows: Vec<T>, id: impl Fn(&T) -> i64) -> HashMap<i64, T> {
    rows.into_iter().map(|row| (id(&row), row)).collect()
}
