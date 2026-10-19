//! Rows of the dictionary as content, for export: one piece per row.

use std::collections::HashMap;

use sea_orm::{ConnectionTrait, DbErr};

use super::row_data::{RowData, load_row_data};
use super::{ContentFilter, Row, matching_rows};
use crate::entity::{label, sentence};
use crate::store::tables::{
    label as label_table, sense as sense_table, sentence as sentence_table,
};
use crate::{Content, ContentProgress, ReadingContent, SenseContent, SentenceContent};

/// Rows matching the filter as content, word by word in creation order, at
/// most `limit` of them. Progress is always given.
pub async fn dump(
    db: &impl ConnectionTrait,
    filter: &ContentFilter,
    limit: Option<u32>,
) -> Result<Vec<Content>, DbErr> {
    let mut rows = matching_rows(db, filter).await?;
    if let Some(limit) = limit {
        rows.truncate(usize::try_from(limit).unwrap_or(usize::MAX));
    }
    let data = load_row_data(db, &rows).await?;
    let sense_ids: Vec<i64> = rows.iter().filter_map(|row| row.sense_id).collect();
    let labels = group(label_table::labels_of_senses(db, &sense_ids).await?, |l| {
        l.sense_id
    });
    let sentences = group(
        sentence_table::sentences_of_senses(db, &sense_ids).await?,
        |s| s.sense_id,
    );
    Ok(rows
        .iter()
        .filter_map(|row| content(row, &data, &labels, &sentences))
        .collect())
}

fn group<T>(rows: Vec<T>, key: impl Fn(&T) -> i64) -> HashMap<i64, Vec<T>> {
    let mut groups: HashMap<i64, Vec<T>> = HashMap::new();
    for row in rows {
        groups.entry(key(&row)).or_default().push(row);
    }
    groups
}

fn content(
    row: &Row,
    data: &RowData,
    labels: &HashMap<i64, Vec<label::Model>>,
    sentences: &HashMap<i64, Vec<sentence::Model>>,
) -> Option<Content> {
    let word = data.words.get(&row.word_id)?;
    let reading = match row.reading_id {
        Some(id) => {
            let reading = data.readings.get(&id)?;
            let sense = match row.sense_id {
                Some(id) => Some(sense_content(
                    data.senses.get(&id)?,
                    labels.get(&id).map(Vec::as_slice).unwrap_or_default(),
                    sentences.get(&id).map(Vec::as_slice).unwrap_or_default(),
                )),
                None => None,
            };
            Some(ReadingContent {
                kana: reading.kana.clone(),
                sense,
            })
        }
        None => None,
    };
    Some(Content {
        writing: word.writing.clone(),
        origin: word.origin.clone(),
        stratum: word.stratum.clone(),
        reading,
    })
}

fn sense_content(
    sense: &crate::entity::sense::Model,
    labels: &[label::Model],
    sentences: &[sentence::Model],
) -> SenseContent {
    let progress = sense_table::progress(sense);
    SenseContent {
        glosses: sense_table::glosses(sense),
        labels: labels.iter().map(|l| l.text.clone()).collect(),
        sentences: sentences
            .iter()
            .map(|s| SentenceContent {
                marked: s.marked.clone(),
                translation: s.translation.clone(),
            })
            .collect(),
        progress: Some(ContentProgress {
            stage: progress.stage,
            passes: progress.passes,
            lapses: progress.lapses,
            modified_at: Some(progress.modified_at),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::query::Completeness;
    use crate::store::tables::label::add_labels;
    use crate::store::tables::reading::insert_reading;
    use crate::store::tables::sense::set_secondary_glosses;
    use crate::store::tables::sentence::insert_sentence;
    use crate::store::tables::word::insert_word;
    use crate::store::test_db::{memory_db, progress_at, sense_of};

    #[tokio::test]
    async fn gives_each_row_once_word_by_word() {
        let db = memory_db().await;
        let ashita = sense_of(&db, "明日", "あした", "明天").await;
        set_secondary_glosses(&db, ashita, &["翌日".to_owned()])
            .await
            .unwrap();
        add_labels(&db, ashita, &["N5".to_owned()]).await.unwrap();
        insert_sentence(&db, ashita, "[[{明日|あした}]]", Some("明天"))
            .await
            .unwrap();
        insert_word(&db, "パソコン", "personal computer", "外来语")
            .await
            .unwrap();
        let ue = insert_word(&db, "上", "", "").await.unwrap();
        insert_reading(&db, ue.id, "うえ").await.unwrap();

        let progress = progress_at(0, 0, 0);
        assert_eq!(
            dump(&db, &ContentFilter::default(), None).await.unwrap(),
            [
                Content {
                    writing: "明日".into(),
                    origin: String::new(),
                    stratum: String::new(),
                    reading: Some(ReadingContent {
                        kana: "あした".into(),
                        sense: Some(SenseContent {
                            glosses: vec!["明天".into(), "翌日".into()],
                            labels: vec!["N5".into()],
                            sentences: vec![SentenceContent {
                                marked: "[[{明日|あした}]]".into(),
                                translation: Some("明天".into()),
                            }],
                            progress: Some(ContentProgress {
                                stage: 0,
                                passes: 0,
                                lapses: 0,
                                modified_at: Some(progress.modified_at),
                            }),
                        }),
                    }),
                },
                Content {
                    writing: "パソコン".into(),
                    origin: "personal computer".into(),
                    stratum: "外来语".into(),
                    reading: None,
                },
                Content {
                    writing: "上".into(),
                    origin: String::new(),
                    stratum: String::new(),
                    reading: Some(ReadingContent {
                        kana: "うえ".into(),
                        sense: None,
                    }),
                },
            ]
        );
    }

    #[tokio::test]
    async fn takes_only_matching_rows_up_to_the_limit() {
        let db = memory_db().await;
        for kana in ["あ", "い", "う"] {
            sense_of(&db, "語", kana, "义").await;
        }
        let with = sense_of(&db, "語", "え", "义").await;
        insert_sentence(&db, with, "[[{語|ご}]]", None)
            .await
            .unwrap();
        let filter = ContentFilter {
            completeness: vec![Completeness::NoSentence],
            ..ContentFilter::default()
        };
        let kana: Vec<String> = dump(&db, &filter, Some(2))
            .await
            .unwrap()
            .into_iter()
            .map(|c| c.reading.unwrap().kana)
            .collect();
        assert_eq!(kana, ["あ", "い"]);
    }
}
