//! Running the command line, and data directories laid out like the app's.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Output, Stdio};

use dictionary::{DictionaryService, ImportOptions};
use listening::{Gender, Language, ListeningService, NewCharacter, NewReference, Role, Uuid};

/// The hand-written standard script in samples/.
pub const SAMPLE_SCRIPT: &str = include_str!("../../../../../samples/script.json");

/// What a run printed and how it ended.
pub struct Run {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
}

/// Runs `noobenkyo` on the data directory, feeding `stdin` if given.
pub fn noobenkyo(data_dir: &Path, args: &[&str], stdin: Option<&str>) -> Run {
    let mut child = Command::new(env!("CARGO_BIN_EXE_noobenkyo"))
        .args(args)
        .env("NOOBENKYO_DATA_DIR", data_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    input.write_all(stdin.unwrap_or("").as_bytes()).unwrap();
    drop(input);
    let Output {
        status,
        stdout,
        stderr,
    } = child.wait_with_output().unwrap();
    Run {
        code: status.code().unwrap(),
        stdout: String::from_utf8(stdout).unwrap(),
        stderr: String::from_utf8(stderr).unwrap(),
    }
}

/// Writes a file in the folder for a command to read.
pub fn file(dir: &Path, name: &str, text: &str) -> String {
    let path = dir.join(name);
    std::fs::write(&path, text).unwrap();
    path.to_string_lossy().into_owned()
}

/// A dictionary holding the given entries, where the app keeps it.
pub async fn dictionary_with(data_dir: &Path, text: &str) {
    DictionaryService::open(&data_dir.join("dictionary"))
        .await
        .unwrap()
        .import(text, ImportOptions::default())
        .await
        .unwrap();
}

/// 星见町 with a narrator, two main characters and a minor one, where the
/// app keeps the listening data.
pub async fn seeded_listening(data_dir: &Path) -> ListeningService {
    let listening = ListeningService::open(&data_dir.join("listening"))
        .await
        .unwrap();
    for name in ["老陈", "美咲", "健二", "店员"] {
        let character = NewCharacter {
            name: name.into(),
            kana: "なまえ".into(),
            gender: Gender::Male,
            age: 30,
            prompt: None,
            references: vec![NewReference {
                language: Language::Ja,
                audio: b"ref.wav".to_vec(),
                text: "参考。".into(),
            }],
        };
        assert!(
            listening
                .create_character(&character)
                .await
                .unwrap()
                .is_empty()
        );
    }
    listening
        .create_world("星见町", "海边的小镇。")
        .await
        .unwrap();
    for (name, role) in [
        ("老陈", Role::Narrator),
        ("美咲", main("高中生。")),
        ("健二", main("同学。")),
        ("店员", Role::Minor),
    ] {
        listening.add_cast("星见町", name, &role).await.unwrap();
    }
    listening
}

/// The sample script stored as an episode of 星见町.
pub async fn sample_episode(listening: &ListeningService) -> Uuid {
    listening
        .import_script("星见町", SAMPLE_SCRIPT)
        .await
        .unwrap()
}

fn main(description: &str) -> Role {
    Role::Main {
        description: description.into(),
    }
}
