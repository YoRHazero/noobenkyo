//! A data directory with the inbox, a seeded world, and the sample script.

use ai::{AiService, ScriptTarget};
use ai_tools::{Delivery, Inbox};
use listening::{Gender, Language, ListeningService, NewCharacter, NewReference, Role};

/// The hand-written standard script in samples/.
pub const SAMPLE_SCRIPT: &str = include_str!("../../../../../samples/script.json");

/// The world `listening` seeds.
pub const WORLD: &str = "星见町";

/// The service over a data directory, and the inbox the command line
/// writes to in that directory.
pub struct TestData {
    pub dir: tempfile::TempDir,
    pub ai: AiService,
    pub inbox: Inbox,
}

impl TestData {
    pub fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        Self {
            ai: AiService::new(dir.path()),
            inbox: Inbox::in_data_dir(dir.path()),
            dir,
        }
    }

    /// 星见町 with 老陈 as narrator, 美咲 and 健二 as main characters and
    /// 店员 as a minor one.
    pub async fn listening(&self) -> ListeningService {
        let listening = ListeningService::open(&self.dir.path().join("listening"))
            .await
            .unwrap();
        for name in ["老陈", "美咲", "健二", "店员"] {
            let problems = listening.create_character(&character(name)).await.unwrap();
            assert!(problems.is_empty(), "{problems:?}");
        }
        listening.create_world(WORLD, "海边的小镇。").await.unwrap();
        for (name, role) in [
            ("老陈", Role::Narrator),
            ("美咲", main_role("高中生，喜欢拍照。")),
            ("健二", main_role("美咲的同学，田径部。")),
            ("店员", Role::Minor),
        ] {
            listening.add_cast(WORLD, name, &role).await.unwrap();
        }
        listening
    }

    pub fn put_entries(&self, name: &str, text: &str) {
        self.inbox
            .put(name, &Delivery::Entries { text: text.into() })
            .unwrap();
    }

    pub fn put_script(&self, name: &str, target: ScriptTarget, text: &str) {
        self.inbox
            .put(
                name,
                &Delivery::Script {
                    target,
                    text: text.into(),
                },
            )
            .unwrap();
    }
}

pub fn new_episode() -> ScriptTarget {
    ScriptTarget::NewEpisode {
        world: WORLD.into(),
    }
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
