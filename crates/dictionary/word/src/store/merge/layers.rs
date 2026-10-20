//! Merges one piece of content into the word tables, layer by layer.

use std::collections::HashSet;

use chrono::{DateTime, FixedOffset};
use sea_orm::{ConnectionTrait, DbErr};

use super::{MergeOutcome, SenseOutcome};
use crate::entity::{reading, word};
use crate::rules::merge::gloss_merge::merge_secondary;
use crate::rules::merge::progress_merge::{later_progress, new_sense_progress};
use crate::rules::merge::sense_match::{SenseMatch, match_sense};
use crate::rules::sentence;
use crate::store::tables::word as word_table;
use crate::store::tables::{label, reading as reading_table, sense, sentence as sentence_table};
use crate::{Content, SenseContent, SentenceContent};

pub async fn merge_content(
    db: &impl ConnectionTrait,
    content: &Content,
    now: DateTime<FixedOffset>,
) -> Result<MergeOutcome, DbErr> {
    let mut outcome = MergeOutcome::default();
    let word = merge_word(db, content, &mut outcome).await?;
    let Some(incoming_reading) = &content.reading else {
        return Ok(outcome);
    };
    let reading = merge_reading(db, word.id, &incoming_reading.kana, &mut outcome).await?;
    let Some(incoming_sense) = &incoming_reading.sense else {
        return Ok(outcome);
    };
    merge_sense(db, reading.id, incoming_sense, now, &mut outcome).await?;
    Ok(outcome)
}

/// Finds or creates the word. An empty stratum is filled in; a set one is kept.
async fn merge_word(
    db: &impl ConnectionTrait,
    content: &Content,
    outcome: &mut MergeOutcome,
) -> Result<word::Model, DbErr> {
    match word_table::find_word(db, &content.writing, &content.origin).await? {
        Some(existing) => {
            if existing.stratum.is_empty() && !content.stratum.is_empty() {
                word_table::set_stratum(db, existing.id, &content.stratum).await?;
            }
            Ok(existing)
        }
        None => {
            outcome.word_added = true;
            word_table::insert_word(db, &content.writing, &content.origin, &content.stratum).await
        }
    }
}

async fn merge_reading(
    db: &impl ConnectionTrait,
    word_id: i64,
    kana: &str,
    outcome: &mut MergeOutcome,
) -> Result<reading::Model, DbErr> {
    match reading_table::find_reading(db, word_id, kana).await? {
        Some(existing) => Ok(existing),
        None => {
            outcome.reading_added = true;
            reading_table::insert_reading(db, word_id, kana).await
        }
    }
}

async fn merge_sense(
    db: &impl ConnectionTrait,
    reading_id: i64,
    incoming: &SenseContent,
    now: DateTime<FixedOffset>,
    outcome: &mut MergeOutcome,
) -> Result<(), DbErr> {
    let existing = sense::senses_of_reading(db, reading_id).await?;
    let gloss_lists: Vec<Vec<String>> = existing.iter().map(sense::glosses).collect();
    let progress = incoming.progress.as_ref();
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

/// Adds sentences whose plain text the sense does not have yet; returns how
/// many. Sentences that do not parse are skipped.
async fn add_sentences(
    db: &impl ConnectionTrait,
    sense_id: i64,
    sentences: &[SentenceContent],
) -> Result<u32, DbErr> {
    let mut plains: HashSet<String> = sentence_table::sentences_of_sense(db, sense_id)
        .await?
        .iter()
        .filter_map(|row| plain_of(&row.marked))
        .collect();
    let mut added = 0;
    for incoming in sentences {
        let Some(plain) = plain_of(&incoming.marked) else {
            continue;
        };
        if plains.insert(plain) {
            sentence_table::insert_sentence(
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

fn plain_of(marked: &str) -> Option<String> {
    sentence::parse(marked)
        .ok()
        .map(|segments| sentence::plain_text(&segments))
}
