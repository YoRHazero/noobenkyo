//! Services in a temporary directory, a seeded world, and the sample script.

use dictionary::{DictionaryService, ImportOptions};
use listening::{Gender, Language, ListeningService, NewCharacter, NewReference, Role};

/// The hand-written standard script in samples/.
pub const SAMPLE_SCRIPT: &str = include_str!("../../../../../samples/script.json");

/// A dictionary holding the given entry text.
pub async fn dictionary_with(dir: &tempfile::TempDir, text: &str) -> DictionaryService {
    let dictionary = DictionaryService::open(&dir.path().join("dictionary"))
        .await
        .unwrap();
    dictionary
        .import(text, ImportOptions::default())
        .await
        .unwrap();
    dictionary
}

pub async fn listening(dir: &tempfile::TempDir) -> ListeningService {
    ListeningService::open(&dir.path().join("listening"))
        .await
        .unwrap()
}

/// 星见町 with 老陈 as narrator, 美咲 and 健二 as main characters and 店员
/// as a minor one; 雾都 with only a main character.
pub async fn seeded(listening: &ListeningService) {
    for name in ["老陈", "美咲", "健二", "店员"] {
        let problems = listening.create_character(&character(name)).await.unwrap();
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
    listening
        .create_world("雾都", "雾很重的城市。")
        .await
        .unwrap();
    listening
        .add_cast("雾都", "美咲", &main_role("侦探。"))
        .await
        .unwrap();
}

fn character(name: &str) -> NewCharacter {
    NewCharacter {
        name: name.into(),
        kana: "なまえ".into(),
        gender: Gender::Female,
        age: 20,
        prompt: None,
        references: vec![NewReference {
            language: Language::Ja,
            audio: b"ref.wav".to_vec(),
            text: "参考。".into(),
        }],
    }
}

fn main_role(description: &str) -> Role {
    Role::Main {
        description: description.into(),
    }
}
