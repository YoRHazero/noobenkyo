//! In-memory database, content builders and table listings.

use chrono::{DateTime, FixedOffset};
use dictionary_word::entity::{label, reading, sense, sentence, word};
use dictionary_word::store::merge::{MergeOutcome, merge_contents};
use dictionary_word::{
    Content, ContentProgress, Migrator, Progress, ReadingContent, SenseContent, SenseKey,
    SentenceContent,
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, EntityTrait, QueryOrder};
use sea_orm_migration::MigratorTrait;

/// An in-memory database with the tables, on a single connection.
pub async fn memory_db() -> DatabaseConnection {
    let mut options = ConnectOptions::new("sqlite::memory:");
    options
        .max_connections(1)
        .min_connections(1)
        .idle_timeout(None)
        .max_lifetime(None)
        .sqlx_logging(false)
        .map_sqlx_sqlite_opts(|opts| opts.foreign_keys(true));
    let db = Database::connect(options).await.unwrap();
    Migrator::up(&db, None).await.unwrap();
    db
}

pub fn time(text: &str) -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339(text).unwrap()
}

/// The moment every merge in these tests happens at.
pub fn now() -> DateTime<FixedOffset> {
    time("2026-10-17T21:05:02.117+08:00")
}

/// Progress stamped `now()`.
pub fn progress_at(stage: u32, passes: u32, lapses: u32) -> Progress {
    Progress {
        stage,
        passes,
        lapses,
        modified_at: now(),
    }
}

pub fn word_only(writing: &str, origin: &str, stratum: &str) -> Content {
    Content {
        writing: writing.into(),
        origin: origin.into(),
        stratum: stratum.into(),
        reading: None,
    }
}

pub fn reading_only(writing: &str, kana: &str) -> Content {
    Content {
        reading: Some(ReadingContent {
            kana: kana.into(),
            sense: None,
        }),
        ..word_only(writing, "", "")
    }
}

/// A sense with the given glosses, nothing else.
pub fn sense(writing: &str, kana: &str, glosses: &[&str]) -> Content {
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

pub fn sense_mut(content: &mut Content) -> &mut SenseContent {
    content.reading.as_mut().unwrap().sense.as_mut().unwrap()
}

pub fn sentence(marked: &str, translation: Option<&str>) -> SentenceContent {
    SentenceContent {
        marked: marked.into(),
        translation: translation.map(str::to_owned),
    }
}

pub fn with_labels(mut content: Content, labels: &[&str]) -> Content {
    sense_mut(&mut content).labels = labels.iter().map(|l| l.to_string()).collect();
    content
}

pub fn with_sentences(mut content: Content, sentences: &[SentenceContent]) -> Content {
    sense_mut(&mut content).sentences = sentences.to_vec();
    content
}

/// Progress modified at the given time.
pub fn with_progress(
    mut content: Content,
    (stage, passes, lapses): (u32, u32, u32),
    modified_at: &str,
) -> Content {
    sense_mut(&mut content).progress = Some(ContentProgress {
        stage,
        passes,
        lapses,
        modified_at: Some(time(modified_at)),
    });
    content
}

/// Merges the contents in order, at `now()`.
pub async fn merge(db: &DatabaseConnection, contents: &[Content]) -> Vec<MergeOutcome> {
    let refs: Vec<&Content> = contents.iter().collect();
    merge_contents(db, &refs, now()).await.unwrap()
}

pub fn sense_key(writing: &str, kana: &str, gloss: &str) -> SenseKey {
    SenseKey {
        writing: writing.into(),
        origin: String::new(),
        kana: kana.into(),
        primary_gloss: gloss.into(),
    }
}

pub async fn all_words(db: &DatabaseConnection) -> Vec<word::Model> {
    word::Entity::find()
        .order_by_asc(word::Column::Id)
        .all(db)
        .await
        .unwrap()
}

pub async fn all_readings(db: &DatabaseConnection) -> Vec<reading::Model> {
    reading::Entity::find()
        .order_by_asc(reading::Column::Id)
        .all(db)
        .await
        .unwrap()
}

pub async fn all_senses(db: &DatabaseConnection) -> Vec<sense::Model> {
    sense::Entity::find()
        .order_by_asc(sense::Column::Id)
        .all(db)
        .await
        .unwrap()
}

pub async fn all_labels(db: &DatabaseConnection) -> Vec<label::Model> {
    label::Entity::find()
        .order_by_asc(label::Column::SenseId)
        .order_by_asc(label::Column::Text)
        .all(db)
        .await
        .unwrap()
}

pub async fn all_sentences(db: &DatabaseConnection) -> Vec<sentence::Model> {
    sentence::Entity::find()
        .order_by_asc(sentence::Column::Id)
        .all(db)
        .await
        .unwrap()
}
