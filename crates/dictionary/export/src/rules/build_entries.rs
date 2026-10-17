//! Builds export entries from all rows: one line per sense; a word or
//! reading without senses is written at its own layer.

use std::collections::HashMap;

use dictionary_import::{Entry, EntrySentence};
use dictionary_word::entity::{label, reading, sense, sentence, word};
use dictionary_word::store::sense::{glosses, progress};

/// All rows of the word tables, each in creation order.
pub struct Rows {
    pub words: Vec<word::Model>,
    pub readings: Vec<reading::Model>,
    pub senses: Vec<sense::Model>,
    pub labels: Vec<label::Model>,
    pub sentences: Vec<sentence::Model>,
}

pub fn build_entries(rows: &Rows, keep_progress: bool) -> Vec<Entry> {
    let readings = group(&rows.readings, |r| r.word_id);
    let senses = group(&rows.senses, |s| s.reading_id);
    let labels = group(&rows.labels, |l| l.sense_id);
    let sentences = group(&rows.sentences, |s| s.sense_id);

    let mut entries = Vec::new();
    for word in &rows.words {
        let base = word_entry(word);
        let Some(word_readings) = readings.get(&word.id) else {
            entries.push(base);
            continue;
        };
        for reading in word_readings {
            let with_kana = Entry {
                kana: Some(reading.kana.clone()),
                ..base.clone()
            };
            let Some(reading_senses) = senses.get(&reading.id) else {
                entries.push(with_kana);
                continue;
            };
            for sense in reading_senses {
                let sense_labels = labels.get(&sense.id).map(Vec::as_slice).unwrap_or_default();
                let sense_sentences = sentences
                    .get(&sense.id)
                    .map(Vec::as_slice)
                    .unwrap_or_default();
                entries.push(sense_entry(
                    &with_kana,
                    sense,
                    sense_labels,
                    sense_sentences,
                    keep_progress,
                ));
            }
        }
    }
    entries
}

fn group<T, F: Fn(&T) -> i64>(rows: &[T], key: F) -> HashMap<i64, Vec<&T>> {
    let mut groups: HashMap<i64, Vec<&T>> = HashMap::new();
    for row in rows {
        groups.entry(key(row)).or_default().push(row);
    }
    groups
}

fn word_entry(word: &word::Model) -> Entry {
    Entry {
        writing: word.writing.clone(),
        origin: non_empty(&word.origin),
        stratum: non_empty(&word.stratum),
        ..Entry::default()
    }
}

fn non_empty(text: &str) -> Option<String> {
    (!text.is_empty()).then(|| text.to_owned())
}

fn sense_entry(
    base: &Entry,
    sense: &sense::Model,
    labels: &[&label::Model],
    sentences: &[&sentence::Model],
    keep_progress: bool,
) -> Entry {
    let mut entry = Entry {
        glosses: Some(glosses(sense)),
        labels: (!labels.is_empty()).then(|| labels.iter().map(|l| l.text.clone()).collect()),
        sentences: (!sentences.is_empty()).then(|| {
            sentences
                .iter()
                .map(|s| EntrySentence {
                    marked: s.marked.clone(),
                    translation: s.translation.clone(),
                })
                .collect()
        }),
        ..base.clone()
    };
    if keep_progress {
        let progress = progress(sense);
        entry.stage = Some(progress.stage);
        entry.passes = Some(progress.passes);
        entry.lapses = Some(progress.lapses);
        entry.modified_at = Some(progress.modified_at);
    }
    entry
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;

    fn rows() -> Rows {
        let time = DateTime::parse_from_rfc3339("2026-10-17T21:05:02.117+08:00").unwrap();
        Rows {
            words: vec![
                word::Model {
                    id: 1,
                    writing: "明日".into(),
                    origin: String::new(),
                    stratum: "和语".into(),
                },
                word::Model {
                    id: 2,
                    writing: "パソコン".into(),
                    origin: "personal computer".into(),
                    stratum: String::new(),
                },
                word::Model {
                    id: 3,
                    writing: "上".into(),
                    origin: String::new(),
                    stratum: String::new(),
                },
            ],
            readings: vec![
                reading::Model {
                    id: 1,
                    word_id: 1,
                    kana: "あした".into(),
                },
                reading::Model {
                    id: 2,
                    word_id: 3,
                    kana: "うえ".into(),
                },
            ],
            senses: vec![sense::Model {
                id: 1,
                reading_id: 1,
                primary_gloss: "明天".into(),
                secondary_glosses: serde_json::json!(["翌日"]),
                stage: 2,
                passes: 3,
                lapses: 1,
                modified_at: time,
            }],
            labels: vec![label::Model {
                sense_id: 1,
                text: "N5".into(),
            }],
            sentences: vec![sentence::Model {
                id: 1,
                sense_id: 1,
                marked: "[[{明日|あした}]]".into(),
                translation: Some("明天".into()),
            }],
        }
    }

    #[test]
    fn writes_each_layer_once() {
        let entries = build_entries(&rows(), true);
        let lines: Vec<String> = entries
            .iter()
            .map(|e| serde_json::to_string(e).unwrap())
            .collect();
        assert_eq!(
            lines,
            [
                r#"{"writing":"明日","stratum":"和语","kana":"あした","glosses":["明天","翌日"],"labels":["N5"],"sentences":[{"marked":"[[{明日|あした}]]","translation":"明天"}],"stage":2,"passes":3,"lapses":1,"modified_at":"2026-10-17T21:05:02.117+08:00"}"#,
                r#"{"writing":"パソコン","origin":"personal computer"}"#,
                r#"{"writing":"上","kana":"うえ"}"#,
            ]
        );
    }

    #[test]
    fn clears_progress_when_asked() {
        let entries = build_entries(&rows(), false);
        assert_eq!(entries[0].stage, None);
        assert_eq!(entries[0].modified_at, None);
    }
}
