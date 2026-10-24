//! Test data and steps shared by the behaviour tests.

use std::path::PathBuf;

use dictionary::{ImportReport, SenseKey};
use listening::{Gender, Language, NewCharacter, NewReference, Role, Uuid};
use noobenkyo_core::app_state::AppState;
use noobenkyo_core::commands::dictionary::entries::{
    dictionary_default_import_options, dictionary_import,
};
use noobenkyo_core::commands::learn::answering::{learn_confirm_answer, learn_submit_answer};
use noobenkyo_core::commands::learn::asking::learn_next;
use noobenkyo_core::commands::listening::characters::listening_create_character;
use noobenkyo_core::commands::listening::script::listening_import_script;
use noobenkyo_core::commands::listening::worlds::{listening_add_cast, listening_create_world};
use noobenkyo_core::commands::review::answering::{review_confirm_answer, review_submit_answer};
use noobenkyo_core::commands::review::asking::review_next;
use noobenkyo_core::payloads::RevealPayload;
use study::Rating;
use tauri::test::{MockRuntime, mock_app};
use tauri::{App, Manager, State};

/// The sample word list, as the user would paste it.
pub const SAMPLE: &str = include_str!("../../../../samples/basic.jsonl");

/// The hand-written standard script: 8 lines spoken by 旁白, 店员, 美咲,
/// 健二 and 路人.
pub const SAMPLE_SCRIPT: &str = include_str!("../../../../samples/script.json");

/// The world `with_world` seeds.
pub const WORLD: &str = "星见町";

/// The app over its own temporary data directory.
pub struct TestApp {
    /// Kept so the directory lives as long as the app.
    dir: tempfile::TempDir,
    app: App<MockRuntime>,
}

impl TestApp {
    /// A fresh app, as at the very first start.
    pub async fn fresh() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let app = mock_app();
        app.manage(AppState::open(&dir.path().join("data")).await.unwrap());
        Self { dir, app }
    }

    /// A fresh app with 星见町 and the sample script stored in it as its
    /// first episode. 老陈 narrates, 美咲 and 健二 are main characters, 店员
    /// is a minor one; all have references in both languages.
    pub async fn with_world() -> (Self, Uuid) {
        let app = Self::fresh().await;
        for (name, kana) in [
            ("老陈", "ろうちん"),
            ("美咲", "みさき"),
            ("健二", "けんじ"),
            ("店员", "てんいん"),
        ] {
            let problems = listening_create_character(app.state(), new_character(name, kana))
                .await
                .unwrap();
            assert!(problems.is_empty(), "{problems:?}");
        }
        listening_create_world(app.state(), WORLD.into(), "海边的小镇。".into())
            .await
            .unwrap();
        for (name, role) in [
            ("老陈", Role::Narrator),
            ("美咲", main_role("高中生，喜欢拍照。")),
            ("健二", main_role("美咲的同学，田径部。")),
            ("店员", Role::Minor),
        ] {
            listening_add_cast(app.state(), WORLD.into(), name.into(), role)
                .await
                .unwrap();
        }
        let episode = listening_import_script(app.state(), WORLD.into(), SAMPLE_SCRIPT.into())
            .await
            .unwrap();
        (app, episode)
    }

    pub fn data_dir(&self) -> PathBuf {
        self.dir.path().join("data")
    }

    /// Leaves an item in the inbox as the command line does: the JSON of one
    /// inbox file, written whole under the item's name.
    pub fn deliver(&self, name: &str, json: &str) {
        let inbox = self.data_dir().join("inbox");
        std::fs::create_dir_all(&inbox).unwrap();
        std::fs::write(inbox.join(format!("{name}.json")), json).unwrap();
    }

    /// A fresh app with the sample word list imported with the default
    /// options: 9 senses with sentences, 上 without, the two katakana words
    /// skipped.
    pub async fn with_sample() -> Self {
        let app = Self::fresh().await;
        let report = app.import_sample().await;
        assert!(report.problems.is_empty(), "{:?}", report.problems);
        app
    }

    pub fn state(&self) -> State<'_, AppState> {
        self.app.state()
    }

    pub async fn import_sample(&self) -> ImportReport {
        dictionary_import(
            self.state(),
            SAMPLE.to_owned(),
            dictionary_default_import_options(),
        )
        .await
        .unwrap()
    }
}

pub fn main_role(description: &str) -> Role {
    Role::Main {
        description: description.into(),
    }
}

/// A twenty-year-old woman with a reference in every given language, as the
/// user would upload them.
pub fn new_character_with(name: &str, kana: &str, languages: &[Language]) -> NewCharacter {
    NewCharacter {
        name: name.into(),
        kana: kana.into(),
        gender: Gender::Female,
        age: 20,
        prompt: None,
        references: languages
            .iter()
            .map(|&language| NewReference {
                language,
                audio: reference_audio(language),
                text: "参考。".into(),
            })
            .collect(),
    }
}

fn new_character(name: &str, kana: &str) -> NewCharacter {
    new_character_with(name, kana, &Language::ALL)
}

/// Stands in for an uploaded WAV file; differs by language so a mix-up shows.
pub fn reference_audio(language: Language) -> Vec<u8> {
    format!("RIFF {language:?} reference").into_bytes()
}

/// The key of the sense a reveal is about. Every sample sense with
/// sentences is a native word, so its origin is empty.
pub fn revealed_key(reveal: &RevealPayload) -> SenseKey {
    SenseKey {
        writing: reveal.view.writing.clone(),
        origin: String::new(),
        kana: reveal.view.kana.clone(),
        primary_gloss: reveal.view.glosses[0].clone(),
    }
}

/// Asks, submits an empty answer and confirms the rating on the learning
/// line, as the frontend does; `false` when there is nothing to learn.
pub async fn learn_once(app: &TestApp, rating: Rating) -> bool {
    if learn_next(app.state()).await.unwrap().is_none() {
        return false;
    }
    learn_submit_answer(app.state(), String::new())
        .await
        .unwrap();
    learn_confirm_answer(app.state(), rating).await.unwrap();
    true
}

/// Learns until nothing is left, rating every quiz as known; the number of
/// quizzes it took.
pub async fn learn_everything(app: &TestApp) -> u32 {
    let mut quizzes = 0;
    while learn_once(app, Rating::Known).await {
        quizzes += 1;
        assert!(quizzes < 100, "learning never ends");
    }
    quizzes
}

/// The same as `learn_once`, in the review session.
pub async fn review_once(app: &TestApp, rating: Rating) -> bool {
    if review_next(app.state()).await.unwrap().is_none() {
        return false;
    }
    review_submit_answer(app.state(), String::new())
        .await
        .unwrap();
    review_confirm_answer(app.state(), rating).await.unwrap();
    true
}
