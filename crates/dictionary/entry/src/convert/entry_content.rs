//! One entry → content, layer by layer.

use dictionary_word::{Content, ReadingContent, SenseContent};

use super::dropped_parts::DroppedParts;
use super::entry_error::EntryError;
use super::sentences::split;
use super::{reading_layer, sense_layer, word_layer};
use crate::Entry;

/// The content of an entry with the parts that were dropped from it, or why
/// the whole entry is dropped.
pub fn entry_content(entry: Entry) -> Result<(Content, DroppedParts), EntryError> {
    let writing = word_layer::writing(&entry.writing)?;
    let origin = word_layer::optional_text(entry.origin);
    let (origin_language, dropped_language) =
        word_layer::origin_language(&origin, entry.origin_language);
    let glosses = sense_layer::glosses(entry.glosses);
    let labels = sense_layer::labels(entry.labels);
    let sentences = entry.sentences.unwrap_or_default();
    let progress =
        sense_layer::progress(entry.stage, entry.passes, entry.lapses, entry.modified_at);

    let needs_glosses = !labels.is_empty() || !sentences.is_empty() || progress.is_some();
    if needs_glosses && glosses.is_none() {
        return Err(EntryError::MissingGlosses);
    }
    let kana = reading_layer::kana(&writing, entry.kana.as_deref(), glosses.is_some())?;

    let (kept, dropped_sentences) = split(sentences);
    let sense = glosses.map(|glosses| SenseContent {
        glosses,
        labels,
        sentences: kept,
        progress,
    });
    let content = Content {
        writing,
        origin,
        origin_language,
        reading: kana.map(|kana| ReadingContent { kana, sense }),
    };
    let dropped = DroppedParts {
        origin_language: dropped_language,
        sentences: dropped_sentences,
    };
    Ok((content, dropped))
}

#[cfg(test)]
mod tests {
    use super::*;
    use dictionary_word::{ContentProgress, SentenceContent};

    fn entry(json: &str) -> Entry {
        serde_json::from_str(json).unwrap()
    }

    fn content_of(json: &str) -> Content {
        entry_content(entry(json)).unwrap().0
    }

    #[test]
    fn writing_only_has_no_reading() {
        let content = content_of(r#"{"writing": "パソコン", "origin": "personal computer"}"#);
        assert_eq!(content.writing, "パソコン");
        assert_eq!(content.origin, "personal computer");
        assert_eq!(content.reading, None);
    }

    #[test]
    fn kana_only_word_derives_reading_when_needed() {
        let content = content_of(r#"{"writing": "コーヒー", "glosses": ["咖啡"]}"#);
        let reading = content.reading.unwrap();
        assert_eq!(reading.kana, "こーひー");
        assert_eq!(reading.sense.unwrap().glosses, ["咖啡"]);
    }

    #[test]
    fn kanji_word_keeps_reading_without_glosses() {
        let content = content_of(r#"{"writing": "明日", "kana": "アス"}"#);
        assert_eq!(
            content.reading,
            Some(ReadingContent {
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
                entry_content(entry(json)).unwrap_err(),
                EntryError::MissingGlosses,
                "{json}"
            );
        }
    }

    #[test]
    fn drops_invalid_sentences_and_keeps_the_rest() {
        let (content, dropped) = entry_content(entry(
            r#"{"writing": "明日", "kana": "あす", "glosses": ["明天"],
                "sentences": [{"marked": "明日"}, {"marked": "[[{明日|あす}]]"}]}"#,
        ))
        .unwrap();
        assert_eq!(dropped.sentences.len(), 1);
        let sense = content.reading.unwrap().sense.unwrap();
        assert_eq!(sense.sentences.len(), 1);
    }

    #[test]
    fn keeps_a_language_only_with_an_origin() {
        let content = content_of(
            r#"{"writing": "コーヒー", "origin": "koffie", "origin_language": "荷兰语"}"#,
        );
        assert_eq!(content.origin_language, "荷兰语");

        let (content, dropped) = entry_content(entry(
            r#"{"writing": "ネコ", "origin_language": "日语", "kana": "ねこ", "glosses": ["猫"]}"#,
        ))
        .unwrap();
        assert_eq!(content.origin_language, "");
        assert_eq!(dropped.origin_language.as_deref(), Some("日语"));
        assert!(content.reading.unwrap().sense.is_some());
    }

    #[test]
    fn keeps_the_sense_when_every_sentence_is_dropped() {
        let (content, dropped) = entry_content(entry(
            r#"{"writing": "明日", "kana": "あす", "glosses": ["明天"], "sentences": [{"marked": "明日"}]}"#,
        ))
        .unwrap();
        assert_eq!(dropped.sentences.len(), 1);
        assert!(content.reading.unwrap().sense.unwrap().sentences.is_empty());
    }

    #[test]
    fn normalizes_full_entry() {
        let content = content_of(
            r#"{"writing": "諦める", "origin": "", "kana": "あきらめる",
                "glosses": ["放弃", "死心", "放弃"], "labels": ["N3"],
                "sentences": [{"marked": "[[{諦|あきら}めない]]で", "translation": " 别放弃 "}],
                "stage": 2, "passes": 3, "modified_at": "2026-10-17T21:05:02.117+08:00"}"#,
        );
        let sense = content.reading.unwrap().sense.unwrap();
        assert_eq!(sense.glosses, ["放弃", "死心"]);
        assert_eq!(sense.labels, ["N3"]);
        assert_eq!(
            sense.sentences,
            [SentenceContent {
                marked: "[[{諦|あきら}めない]]で".into(),
                translation: Some("别放弃".into()),
            }]
        );
        let progress: ContentProgress = sense.progress.unwrap();
        assert_eq!(
            (progress.stage, progress.passes, progress.lapses),
            (2, 3, 0)
        );
        assert!(progress.modified_at.is_some());
    }
}
