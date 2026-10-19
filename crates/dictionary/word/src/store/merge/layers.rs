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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::tables::label::all_labels;
    use crate::store::tables::sense::all_senses;
    use crate::store::tables::sentence::all_sentences;
    use crate::store::tables::word::all_words;
    use crate::store::test_db::{memory_db, time};
    use crate::{ContentProgress, ReadingContent};

    fn now() -> DateTime<FixedOffset> {
        time("2026-10-18T09:00:00+08:00")
    }

    fn word_only(writing: &str, origin: &str, stratum: &str) -> Content {
        Content {
            writing: writing.into(),
            origin: origin.into(),
            stratum: stratum.into(),
            reading: None,
        }
    }

    fn sense_content(writing: &str, kana: &str, glosses: &[&str]) -> Content {
        Content {
            reading: Some(ReadingContent {
                kana: kana.into(),
                sense: Some(SenseContent {
                    glosses: glosses.iter().map(|g| g.to_string()).collect(),
                    labels: vec![],
                    sentences: vec![],
                    progress: None,
                }),
            }),
            ..word_only(writing, "", "")
        }
    }

    fn sense_mut(content: &mut Content) -> &mut SenseContent {
        content.reading.as_mut().unwrap().sense.as_mut().unwrap()
    }

    fn sentence(marked: &str, translation: Option<&str>) -> SentenceContent {
        SentenceContent {
            marked: marked.into(),
            translation: translation.map(str::to_owned),
        }
    }

    fn with_progress(mut content: Content, stage: u32, modified_at: &str) -> Content {
        sense_mut(&mut content).progress = Some(ContentProgress {
            stage,
            passes: 0,
            lapses: 0,
            modified_at: Some(time(modified_at)),
        });
        content
    }

    async fn merge(db: &impl ConnectionTrait, content: &Content) -> MergeOutcome {
        merge_content(db, content, now()).await.unwrap()
    }

    #[tokio::test]
    async fn creates_every_layer() {
        let db = memory_db().await;
        let mut content = sense_content("明日", "あした", &["明天"]);
        sense_mut(&mut content).labels = vec!["N5".into()];
        sense_mut(&mut content).sentences = vec![sentence("[[{明日|あした}]]", None)];
        assert_eq!(
            merge(&db, &content).await,
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
        let outcome = merge(&db, &word_only("パソコン", "personal computer", "")).await;
        assert!(outcome.word_added && !outcome.reading_added);
        assert_eq!(all_words(&db).await.unwrap()[0].origin, "personal computer");
    }

    #[tokio::test]
    async fn merges_intersecting_glosses_into_secondary() {
        let db = memory_db().await;
        let mut first = sense_content("諦める", "あきらめる", &["放弃"]);
        sense_mut(&mut first).labels = vec!["N3".into()];
        merge(&db, &first).await;
        let mut second = sense_content("諦める", "あきらめる", &["死心", "放弃"]);
        sense_mut(&mut second).labels = vec!["动词".into()];
        assert_eq!(merge(&db, &second).await.sense, SenseOutcome::Merged);
        let senses = all_senses(&db).await.unwrap();
        assert_eq!(senses.len(), 1);
        assert_eq!(sense::glosses(&senses[0]), ["放弃", "死心"]);
        assert_eq!(all_labels(&db).await.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn reports_conflict_without_changes() {
        let db = memory_db().await;
        merge(&db, &sense_content("上", "うえ", &["上面"])).await;
        merge(&db, &sense_content("上", "うえ", &["上级"])).await;
        let outcome = merge(&db, &sense_content("上", "うえ", &["上面", "上级"])).await;
        assert_eq!(
            outcome.sense,
            SenseOutcome::Conflict(vec!["上面".into(), "上级".into()])
        );
        assert_eq!(all_senses(&db).await.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn keeps_existing_sentence_with_same_plain_text() {
        let db = memory_db().await;
        let mut first = sense_content("明日", "あした", &["明天"]);
        sense_mut(&mut first).sentences = vec![sentence("[[{明日|あした}]]だ", Some("旧"))];
        merge(&db, &first).await;
        let mut second = sense_content("明日", "あした", &["明天"]);
        sense_mut(&mut second).sentences = vec![
            sentence("[[{明日|あした}]]だ", Some("新")),
            sentence("[[{明日|あした}]]よ", None),
        ];
        assert_eq!(merge(&db, &second).await.sentences_added, 1);
        let sentences = all_sentences(&db).await.unwrap();
        assert_eq!(sentences[0].translation.as_deref(), Some("旧"));
        assert_eq!(sentences.len(), 2);
    }

    #[tokio::test]
    async fn takes_later_progress() {
        let db = memory_db().await;
        let base = sense_content("明日", "あした", &["明天"]);
        merge(
            &db,
            &with_progress(base.clone(), 1, "2026-10-01T00:00:00+08:00"),
        )
        .await;
        let older = merge(
            &db,
            &with_progress(base.clone(), 5, "2026-09-01T00:00:00+08:00"),
        )
        .await;
        assert!(!older.progress_updated);
        let newer = merge(&db, &with_progress(base, 3, "2026-10-02T00:00:00+08:00")).await;
        assert!(newer.progress_updated);
        assert_eq!(sense::progress(&all_senses(&db).await.unwrap()[0]).stage, 3);
    }

    #[tokio::test]
    async fn fills_empty_stratum_only() {
        let db = memory_db().await;
        merge(&db, &word_only("明日", "", "")).await;
        merge(&db, &word_only("明日", "", "和语")).await;
        merge(&db, &word_only("明日", "", "汉语")).await;
        assert_eq!(all_words(&db).await.unwrap()[0].stratum, "和语");
    }
}
