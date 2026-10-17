//! Turns a raw entry into a normalized, layered `ValidEntry`.

use dictionary_word::rules::kana::normalize_reading;
use dictionary_word::rules::kanji::has_kanji;
use dictionary_word::rules::marked;
use dictionary_word::rules::writing::normalize_writing;

use super::entry_error::EntryError;
use super::text_list::clean_list;
use crate::{
    Entry, EntryProgress, EntrySentence, ValidEntry, ValidReading, ValidSense, ValidSentence,
};

pub fn validate(entry: Entry) -> Result<ValidEntry, EntryError> {
    let writing = normalize_writing(&entry.writing)?;
    let glosses = entry
        .glosses
        .map(clean_list)
        .filter(|glosses| !glosses.is_empty());
    let labels = clean_list(entry.labels.unwrap_or_default());
    let sentences = entry.sentences.unwrap_or_default();
    let progress = entry_progress(entry.stage, entry.passes, entry.lapses, entry.modified_at);

    let needs_glosses = !labels.is_empty() || !sentences.is_empty() || progress.is_some();
    if needs_glosses && glosses.is_none() {
        return Err(EntryError::MissingGlosses);
    }
    let kana = resolve_kana(&writing, entry.kana.as_deref(), glosses.is_some())?;

    let sense = match glosses {
        Some(glosses) => Some(ValidSense {
            glosses,
            labels,
            sentences: validate_sentences(sentences)?,
            progress,
        }),
        None => None,
    };
    Ok(ValidEntry {
        writing,
        origin: trimmed(entry.origin),
        stratum: trimmed(entry.stratum),
        reading: kana.map(|kana| ValidReading { kana, sense }),
    })
}

fn trimmed(value: Option<String>) -> String {
    value.map(|v| v.trim().to_owned()).unwrap_or_default()
}

fn entry_progress(
    stage: Option<u32>,
    passes: Option<u32>,
    lapses: Option<u32>,
    modified_at: Option<chrono::DateTime<chrono::FixedOffset>>,
) -> Option<EntryProgress> {
    if stage.is_none() && passes.is_none() && lapses.is_none() && modified_at.is_none() {
        return None;
    }
    Some(EntryProgress {
        stage: stage.unwrap_or(0),
        passes: passes.unwrap_or(0),
        lapses: lapses.unwrap_or(0),
        modified_at,
    })
}

/// Words with kanji take the given kana; words without kanji derive it from the writing.
fn resolve_kana(
    writing: &str,
    given: Option<&str>,
    needed: bool,
) -> Result<Option<String>, EntryError> {
    if has_kanji(writing) {
        return match given {
            Some(raw) => Ok(Some(normalize_reading(raw)?)),
            None if needed => Err(EntryError::MissingKana),
            None => Ok(None),
        };
    }
    match given {
        Some(raw) => {
            let kana = normalize_reading(raw)?;
            let expected = normalize_reading(writing)?;
            if kana == expected {
                Ok(Some(kana))
            } else {
                Err(EntryError::KanaMismatch { expected })
            }
        }
        None if needed => Ok(Some(normalize_reading(writing)?)),
        None => Ok(None),
    }
}

/// Checks every sentence and drops later ones whose plain text repeats.
fn validate_sentences(sentences: Vec<EntrySentence>) -> Result<Vec<ValidSentence>, EntryError> {
    let mut valid: Vec<ValidSentence> = Vec::with_capacity(sentences.len());
    for sentence in sentences {
        let marked_text = sentence.marked.trim().to_owned();
        let segments =
            marked::parse_checked(&marked_text).map_err(|error| EntryError::Sentence {
                marked: marked_text.clone(),
                error,
            })?;
        let plain = marked::plain_text(&segments);
        if valid.iter().any(|kept| kept.plain == plain) {
            continue;
        }
        valid.push(ValidSentence {
            marked: marked_text,
            plain,
            translation: sentence
                .translation
                .map(|t| t.trim().to_owned())
                .filter(|t| !t.is_empty()),
        });
    }
    Ok(valid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dictionary_word::rules::marked::MarkedError;

    fn entry(json: &str) -> Entry {
        serde_json::from_str(json).unwrap()
    }

    #[test]
    fn writing_only_has_no_reading() {
        let valid = validate(entry(r#"{"writing": "パソコン"}"#)).unwrap();
        assert_eq!(valid.writing, "パソコン");
        assert_eq!(valid.origin, "");
        assert_eq!(valid.reading, None);
    }

    #[test]
    fn kana_only_word_derives_reading_when_needed() {
        let valid = validate(entry(r#"{"writing": "コーヒー", "glosses": ["咖啡"]}"#)).unwrap();
        let reading = valid.reading.unwrap();
        assert_eq!(reading.kana, "こーひー");
        assert_eq!(reading.sense.unwrap().glosses, ["咖啡"]);
    }

    #[test]
    fn kana_only_word_rejects_other_reading() {
        let error = validate(entry(r#"{"writing": "きれい", "kana": "きれ"}"#)).unwrap_err();
        assert_eq!(
            error,
            EntryError::KanaMismatch {
                expected: "きれい".into()
            }
        );
    }

    #[test]
    fn kanji_word_needs_kana_for_glosses() {
        let error = validate(entry(r#"{"writing": "明日", "glosses": ["明天"]}"#)).unwrap_err();
        assert_eq!(error, EntryError::MissingKana);
    }

    #[test]
    fn kanji_word_keeps_reading_without_glosses() {
        let valid = validate(entry(r#"{"writing": "明日", "kana": "アス"}"#)).unwrap();
        assert_eq!(
            valid.reading,
            Some(ValidReading {
                kana: "あす".into(),
                sense: None
            })
        );
    }

    #[test]
    fn deeper_fields_need_glosses() {
        for json in [
            r#"{"writing": "明日", "kana": "あす", "labels": ["N5"]}"#,
            r#"{"writing": "明日", "kana": "あす", "sentences": [{"marked": "[[{明日|あす}]]"}]}"#,
            r#"{"writing": "明日", "kana": "あす", "stage": 1}"#,
            r#"{"writing": "明日", "kana": "あす", "glosses": [" "], "labels": ["N5"]}"#,
        ] {
            assert_eq!(
                validate(entry(json)).unwrap_err(),
                EntryError::MissingGlosses,
                "{json}"
            );
        }
    }

    #[test]
    fn rejects_invalid_sentence() {
        let error = validate(entry(
            r#"{"writing": "明日", "kana": "あす", "glosses": ["明天"], "sentences": [{"marked": "明日"}]}"#,
        ))
        .unwrap_err();
        assert_eq!(
            error,
            EntryError::Sentence {
                marked: "明日".into(),
                error: MarkedError::TargetCount(0)
            }
        );
    }

    #[test]
    fn normalizes_full_entry() {
        let valid = validate(entry(
            r#"{"writing": "諦める", "origin": "", "stratum": "和语", "kana": "あきらめる",
                "glosses": ["放弃", "死心", "放弃"], "labels": ["N3"],
                "sentences": [
                    {"marked": "[[{諦|あきら}めない]]で", "translation": " 别放弃 "},
                    {"marked": "[[{諦|あきら}めない]]で", "translation": "重复"}
                ],
                "stage": 2, "passes": 3, "modified_at": "2026-10-17T21:05:02.117+08:00"}"#,
        ))
        .unwrap();
        assert_eq!(valid.stratum, "和语");
        let sense = valid.reading.unwrap().sense.unwrap();
        assert_eq!(sense.glosses, ["放弃", "死心"]);
        assert_eq!(sense.labels, ["N3"]);
        assert_eq!(
            sense.sentences,
            [ValidSentence {
                marked: "[[{諦|あきら}めない]]で".into(),
                plain: "諦めないで".into(),
                translation: Some("别放弃".into()),
            }]
        );
        let progress = sense.progress.unwrap();
        assert_eq!(
            (progress.stage, progress.passes, progress.lapses),
            (2, 3, 0)
        );
        assert!(progress.modified_at.is_some());
    }

    #[test]
    fn rejects_unknown_fields() {
        assert!(
            serde_json::from_str::<Entry>(r#"{"writing": "明日", "gloss": ["明天"]}"#).is_err()
        );
    }
}
