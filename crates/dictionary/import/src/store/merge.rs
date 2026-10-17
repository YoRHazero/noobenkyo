//! Merges one valid entry into the word tables, layer by layer.

use std::collections::HashSet;

use chrono::{DateTime, FixedOffset};
use dictionary_word::entity::{reading, word};
use dictionary_word::rules::marked;
use dictionary_word::store::{
    label, reading as reading_store, sense, sentence, word as word_store,
};
use sea_orm::{ConnectionTrait, DbErr};

use super::merge_outcome::{MergeOutcome, SenseOutcome};
use crate::rules::gloss_merge::merge_secondary;
use crate::rules::progress_merge::{later_progress, new_sense_progress};
use crate::rules::sense_match::{SenseMatch, match_sense};
use crate::{EntryProgress, ValidEntry, ValidSense, ValidSentence};

pub async fn merge_entry(
    db: &impl ConnectionTrait,
    entry: &ValidEntry,
    keep_progress: bool,
    now: DateTime<FixedOffset>,
) -> Result<MergeOutcome, DbErr> {
    let mut outcome = MergeOutcome::default();
    let word = merge_word(db, entry, &mut outcome).await?;
    let Some(valid_reading) = &entry.reading else {
        return Ok(outcome);
    };
    let reading = merge_reading(db, word.id, &valid_reading.kana, &mut outcome).await?;
    let Some(valid_sense) = &valid_reading.sense else {
        return Ok(outcome);
    };
    let progress = if keep_progress {
        valid_sense.progress.as_ref()
    } else {
        None
    };
    merge_sense(db, reading.id, valid_sense, progress, now, &mut outcome).await?;
    Ok(outcome)
}

/// Finds or creates the word. An empty stratum is filled in; a set one is kept.
async fn merge_word(
    db: &impl ConnectionTrait,
    entry: &ValidEntry,
    outcome: &mut MergeOutcome,
) -> Result<word::Model, DbErr> {
    match word_store::find_word(db, &entry.writing, &entry.origin).await? {
        Some(existing) => {
            if existing.stratum.is_empty() && !entry.stratum.is_empty() {
                word_store::set_stratum(db, existing.id, &entry.stratum).await?;
            }
            Ok(existing)
        }
        None => {
            outcome.word_added = true;
            word_store::insert_word(db, &entry.writing, &entry.origin, &entry.stratum).await
        }
    }
}

async fn merge_reading(
    db: &impl ConnectionTrait,
    word_id: i64,
    kana: &str,
    outcome: &mut MergeOutcome,
) -> Result<reading::Model, DbErr> {
    match reading_store::find_reading(db, word_id, kana).await? {
        Some(existing) => Ok(existing),
        None => {
            outcome.reading_added = true;
            reading_store::insert_reading(db, word_id, kana).await
        }
    }
}

async fn merge_sense(
    db: &impl ConnectionTrait,
    reading_id: i64,
    incoming: &ValidSense,
    progress: Option<&EntryProgress>,
    now: DateTime<FixedOffset>,
    outcome: &mut MergeOutcome,
) -> Result<(), DbErr> {
    let existing = sense::senses_of_reading(db, reading_id).await?;
    let gloss_lists: Vec<Vec<String>> = existing.iter().map(sense::glosses).collect();
    match match_sense(&gloss_lists, &incoming.glosses) {
        SenseMatch::New => {
            let created = sense::insert_sense(
                db,
                reading_id,
                &incoming.glosses[0],
                &incoming.glosses[1..],
                &new_sense_progress(progress, now),
            )
            .await?;
            label::add_labels(db, created.id, &incoming.labels).await?;
            outcome.sentences_added = add_sentences(db, created.id, &incoming.sentences).await?;
            outcome.sense = SenseOutcome::Added;
        }
        SenseMatch::Existing(index) => {
            let target = &existing[index];
            let secondary = sense::secondary_glosses(target);
            let merged = merge_secondary(&target.primary_gloss, &secondary, &incoming.glosses);
            if merged != secondary {
                sense::set_secondary_glosses(db, target.id, &merged).await?;
            }
            label::add_labels(db, target.id, &incoming.labels).await?;
            outcome.sentences_added = add_sentences(db, target.id, &incoming.sentences).await?;
            if let Some(later) = progress.and_then(|p| later_progress(&sense::progress(target), p))
            {
                sense::set_progress(db, target.id, &later).await?;
                outcome.progress_updated = true;
            }
            outcome.sense = SenseOutcome::Merged;
        }
        SenseMatch::Conflict(indices) => {
            let primaries = indices
                .iter()
                .map(|i| existing[*i].primary_gloss.clone())
                .collect();
            outcome.sense = SenseOutcome::Conflict(primaries);
        }
    }
    Ok(())
}

/// Adds sentences whose plain text the sense does not have yet; returns how many.
async fn add_sentences(
    db: &impl ConnectionTrait,
    sense_id: i64,
    sentences: &[ValidSentence],
) -> Result<u32, DbErr> {
    let mut plains: HashSet<String> = sentence::sentences_of_sense(db, sense_id)
        .await?
        .iter()
        .filter_map(|row| marked::parse(&row.marked).ok())
        .map(|segments| marked::plain_text(&segments))
        .collect();
    let mut added = 0;
    for incoming in sentences {
        if plains.insert(incoming.plain.clone()) {
            sentence::insert_sentence(
                db,
                sense_id,
                &incoming.marked,
                incoming.translation.as_deref(),
            )
            .await?;
            added += 1;
        }
    }
    Ok(added)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::validate::validate;
    use crate::store::test_db::memory_db;
    use dictionary_word::store::{
        label::all_labels, sense::all_senses, sentence::all_sentences, word::all_words,
    };

    fn now() -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339("2026-10-18T09:00:00+08:00").unwrap()
    }

    async fn merge(db: &impl ConnectionTrait, json: &str) -> MergeOutcome {
        let entry = validate(serde_json::from_str(json).unwrap()).unwrap();
        merge_entry(db, &entry, true, now()).await.unwrap()
    }

    #[tokio::test]
    async fn creates_every_layer() {
        let db = memory_db().await;
        let outcome = merge(
            &db,
            r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"], "labels": ["N5"],
                "sentences": [{"marked": "[[{明日|あした}]]"}]}"#,
        )
        .await;
        assert_eq!(
            outcome,
            MergeOutcome {
                word_added: true,
                reading_added: true,
                sense: SenseOutcome::Added,
                sentences_added: 1,
                progress_updated: false,
            }
        );
        let senses = all_senses(&db).await.unwrap();
        assert_eq!(senses[0].primary_gloss, "明天");
        assert_eq!(sense::progress(&senses[0]).modified_at, now());
        assert_eq!(all_labels(&db).await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn stops_at_the_given_layer() {
        let db = memory_db().await;
        let outcome = merge(
            &db,
            r#"{"writing": "パソコン", "origin": "personal computer"}"#,
        )
        .await;
        assert!(outcome.word_added && !outcome.reading_added);
        assert_eq!(all_words(&db).await.unwrap()[0].origin, "personal computer");
    }

    #[tokio::test]
    async fn merges_intersecting_glosses_into_secondary() {
        let db = memory_db().await;
        merge(
            &db,
            r#"{"writing": "諦める", "kana": "あきらめる", "glosses": ["放弃"], "labels": ["N3"]}"#,
        )
        .await;
        let outcome = merge(
            &db,
            r#"{"writing": "諦める", "kana": "あきらめる", "glosses": ["死心", "放弃"], "labels": ["动词"]}"#,
        )
        .await;
        assert_eq!(outcome.sense, SenseOutcome::Merged);
        let senses = all_senses(&db).await.unwrap();
        assert_eq!(senses.len(), 1);
        assert_eq!(sense::glosses(&senses[0]), ["放弃", "死心"]);
        assert_eq!(all_labels(&db).await.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn reports_conflict_without_changes() {
        let db = memory_db().await;
        merge(
            &db,
            r#"{"writing": "上", "kana": "うえ", "glosses": ["上面"]}"#,
        )
        .await;
        merge(
            &db,
            r#"{"writing": "上", "kana": "うえ", "glosses": ["上级"]}"#,
        )
        .await;
        let outcome = merge(
            &db,
            r#"{"writing": "上", "kana": "うえ", "glosses": ["上面", "上级"]}"#,
        )
        .await;
        assert_eq!(
            outcome.sense,
            SenseOutcome::Conflict(vec!["上面".into(), "上级".into()])
        );
        assert_eq!(all_senses(&db).await.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn keeps_existing_sentence_with_same_plain_text() {
        let db = memory_db().await;
        merge(
            &db,
            r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"],
                "sentences": [{"marked": "[[{明日|あした}]]だ", "translation": "旧"}]}"#,
        )
        .await;
        let outcome = merge(
            &db,
            r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"],
                "sentences": [{"marked": "[[{明日|あした}]]だ", "translation": "新"}, {"marked": "[[{明日|あした}]]よ"}]}"#,
        )
        .await;
        assert_eq!(outcome.sentences_added, 1);
        let sentences = all_sentences(&db).await.unwrap();
        assert_eq!(sentences[0].translation.as_deref(), Some("旧"));
        assert_eq!(sentences.len(), 2);
    }

    #[tokio::test]
    async fn takes_later_progress() {
        let db = memory_db().await;
        merge(&db, r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"], "stage": 1, "modified_at": "2026-10-01T00:00:00+08:00"}"#).await;
        let older = merge(&db, r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"], "stage": 5, "modified_at": "2026-09-01T00:00:00+08:00"}"#).await;
        assert!(!older.progress_updated);
        let newer = merge(&db, r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"], "stage": 3, "modified_at": "2026-10-02T00:00:00+08:00"}"#).await;
        assert!(newer.progress_updated);
        assert_eq!(sense::progress(&all_senses(&db).await.unwrap()[0]).stage, 3);
    }

    #[tokio::test]
    async fn ignores_progress_when_not_kept() {
        let db = memory_db().await;
        let entry = validate(
            serde_json::from_str(
                r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"], "stage": 4}"#,
            )
            .unwrap(),
        )
        .unwrap();
        merge_entry(&db, &entry, false, now()).await.unwrap();
        assert_eq!(sense::progress(&all_senses(&db).await.unwrap()[0]).stage, 0);
    }

    #[tokio::test]
    async fn fills_empty_stratum_only() {
        let db = memory_db().await;
        merge(&db, r#"{"writing": "明日"}"#).await;
        merge(&db, r#"{"writing": "明日", "stratum": "和语"}"#).await;
        merge(&db, r#"{"writing": "明日", "stratum": "汉语"}"#).await;
        assert_eq!(all_words(&db).await.unwrap()[0].stratum, "和语");
    }
}
