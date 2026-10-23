//! In-memory database, sample values and shortcuts.

use chrono::{DateTime, FixedOffset};
use listening_drama::character::store::create::create_character;
use listening_drama::episode::store::detail::episode;
use listening_drama::script::store::import::import_script;
use listening_drama::world::store::cast::add::add_cast;
use listening_drama::world::store::create::create_world;
use listening_drama::{
    Gender, Language, LineView, Migrator, NewCharacter, NewReference, Role, Script, ScriptLine,
};
use rand::SeedableRng;
use rand::rngs::StdRng;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use uuid::Uuid;

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

pub fn reference(language: Language, audio: &str, text: &str) -> NewReference {
    NewReference {
        language,
        audio: audio.as_bytes().to_vec(),
        text: text.into(),
    }
}

/// A twenty-year-old woman with a Japanese reference recording.
pub fn new_character(name: &str, kana: &str) -> NewCharacter {
    NewCharacter {
        name: name.into(),
        kana: kana.into(),
        gender: Gender::Female,
        age: 20,
        prompt: None,
        references: vec![reference(Language::Ja, "ja.wav", "こんにちは。")],
    }
}

/// Creates a character that must pass the checks.
pub async fn create(db: &DatabaseConnection, new: &NewCharacter) {
    let problems = create_character(db, new).await.unwrap();
    assert!(problems.is_empty(), "{problems:?}");
}

pub fn rng() -> StdRng {
    StdRng::seed_from_u64(7)
}

/// The moment every script in these tests is stored at.
pub fn now() -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339("2026-10-23T21:05:02.117+08:00").unwrap()
}

pub fn main_role(description: &str) -> Role {
    Role::Main {
        description: description.into(),
    }
}

/// A world named 星见町 whose cast has 老陈 as narrator, 美咲 and 健二 as
/// main characters, and 店员 as a minor one. Everyone has Japanese and
/// Chinese references except 健二, who has Japanese only.
pub async fn seeded_world(db: &DatabaseConnection) {
    let both = vec![
        reference(Language::Ja, "ja.wav", "こんにちは。"),
        reference(Language::Zh, "zh.wav", "你好。"),
    ];
    for (name, kana) in [
        ("老陈", "ろうちん"),
        ("美咲", "みさき"),
        ("店员", "てんいん"),
    ] {
        create(
            db,
            &NewCharacter {
                references: both.clone(),
                ..new_character(name, kana)
            },
        )
        .await;
    }
    create(db, &new_character("健二", "けんじ")).await;
    create_world(db, "星见町", "海边的小镇。").await.unwrap();
    for (name, role) in [
        ("老陈", Role::Narrator),
        ("美咲", main_role("高中生，开朗。")),
        ("健二", main_role("美咲的同学。")),
        ("店员", Role::Minor),
    ] {
        add_cast(db, "星见町", name, &role).await.unwrap();
    }
}

pub fn script(title: &str, lines: &[(&str, &str, &str)]) -> Script {
    Script {
        title: title.into(),
        summary: format!("{title}的梗概。"),
        lines: lines
            .iter()
            .map(|(speaker, ja, zh)| ScriptLine {
                speaker: (*speaker).into(),
                ja: (*ja).into(),
                zh: (*zh).into(),
            })
            .collect(),
    }
}

/// A short script with the narrator, a main character and a passer-by.
pub fn first_script() -> Script {
    script(
        "初遇",
        &[
            ("旁白", "{春|はる}の{朝|あさ}。", "春天的早晨。"),
            ("美咲", "おはよう！", "早上好！"),
            ("路人", "いい{天気|てんき}だね。", "天气真好。"),
        ],
    )
}

/// Stores a script as an episode of 星见町 with the uuid made from `n`.
pub async fn import(db: &DatabaseConnection, script: &Script, n: u128) -> Uuid {
    let uuid = Uuid::from_u128(n);
    import_script(db, "星见町", script, uuid, now(), &mut rng())
        .await
        .unwrap();
    uuid
}

/// The lines of an episode that must exist.
pub async fn lines(db: &DatabaseConnection, uuid: Uuid) -> Vec<LineView> {
    episode(db, uuid).await.unwrap().unwrap().lines
}
