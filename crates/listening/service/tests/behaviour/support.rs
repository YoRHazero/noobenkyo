//! A service in a temporary directory, a seeded world, and the sample script.

use listening::{Gender, Language, ListeningService, NewCharacter, NewReference, Role, Uuid};

/// The hand-written standard script in samples/.
pub const SAMPLE_SCRIPT: &str = include_str!("../../../../../samples/script.json");

pub async fn service(dir: &tempfile::TempDir) -> ListeningService {
    ListeningService::open(dir.path()).await.unwrap()
}

pub fn reference(language: Language, audio: &str, text: &str) -> NewReference {
    NewReference {
        language,
        audio: audio.as_bytes().to_vec(),
        text: text.into(),
    }
}

pub fn new_character(name: &str, kana: &str, languages: &[Language]) -> NewCharacter {
    NewCharacter {
        name: name.into(),
        kana: kana.into(),
        gender: Gender::Female,
        age: 20,
        prompt: None,
        references: languages
            .iter()
            .map(|language| reference(*language, "ref.wav", "参考。"))
            .collect(),
    }
}

pub fn main_role(description: &str) -> Role {
    Role::Main {
        description: description.into(),
    }
}

/// 星见町 with 老陈 as narrator, 美咲 and 健二 as main characters and 店员
/// as a minor one. Everyone speaks both languages except 健二 (Japanese only).
pub async fn seeded(listening: &ListeningService) {
    let both = [Language::Ja, Language::Zh];
    for (name, kana, languages) in [
        ("老陈", "ろうちん", &both[..]),
        ("美咲", "みさき", &both[..]),
        ("健二", "けんじ", &both[..1]),
        ("店员", "てんいん", &both[..]),
    ] {
        let problems = listening
            .create_character(&new_character(name, kana, languages))
            .await
            .unwrap();
        assert!(problems.is_empty(), "{problems:?}");
    }
    listening
        .create_world("星见町", "海边的小镇。")
        .await
        .unwrap();
    for (name, role) in [
        ("老陈", Role::Narrator),
        ("美咲", main_role("高中生，喜欢拍照。")),
        ("健二", main_role("美咲的同学，田径部。")),
        ("店员", Role::Minor),
    ] {
        listening.add_cast("星见町", name, &role).await.unwrap();
    }
}

/// The sample script stored in 星见町.
pub async fn sample_episode(listening: &ListeningService) -> Uuid {
    listening
        .import_script("星见町", SAMPLE_SCRIPT)
        .await
        .unwrap()
}
