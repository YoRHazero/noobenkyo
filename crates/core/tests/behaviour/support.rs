//! Test data and steps shared by the behaviour tests.

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::Duration;

use ai::{AiSettings, InvokerSettings, Machine, ModelChoice, QwenTtsModel, Runtime, TtsOp};
use dictionary::{ImportReport, SenseKey};
use inbox::InboxSource;
use listening::{Gender, Language, NewCharacter, NewReference, Role, Uuid};
use noobenkyo_core::app_state::AppState;
use noobenkyo_core::bindings;
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
use noobenkyo_core::events::ai_task_finished::AiTaskFinished;
use noobenkyo_core::events::ai_task_progress::AiTaskProgress;
use noobenkyo_core::payloads::RevealPayload;
use study::Rating;
use tauri::test::{MockRuntime, mock_app};
use tauri::{App, AppHandle, Manager, State};
use tauri_specta::Event;
use tokio::sync::mpsc::{UnboundedReceiver, unbounded_channel};

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
        tauri_specta::Builder::<MockRuntime>::new()
            .events(bindings::events())
            .mount_events(&app);
        app.manage(AppState::open(&dir.path().join("data")).await.unwrap());
        Self { dir, app }
    }

    /// A fresh app with 星见町 and the sample script stored in it as its
    /// first episode. 老陈 narrates, 美咲 and 健二 are main characters, 店员
    /// is a minor one; all have references in both languages.
    pub async fn with_world() -> (Self, Uuid) {
        let app = Self::fresh().await;
        let episode = app
            .seed_world(|language| NewReference {
                language,
                audio: reference_audio(language),
                text: "参考。".into(),
            })
            .await;
        (app, episode)
    }

    /// Stores what `with_world` has, every character with the references
    /// `reference` gives. Returns the episode.
    pub async fn seed_world(&self, reference: impl Fn(Language) -> NewReference) -> Uuid {
        for (name, kana) in [
            ("老陈", "ろうちん"),
            ("美咲", "みさき"),
            ("健二", "けんじ"),
            ("店员", "てんいん"),
        ] {
            let mut character = new_character_with(name, kana, &[]);
            character.references = Language::ALL.map(&reference).to_vec();
            let problems = listening_create_character(self.state(), character)
                .await
                .unwrap();
            assert!(problems.is_empty(), "{problems:?}");
        }
        listening_create_world(self.state(), WORLD.into(), "海边的小镇。".into())
            .await
            .unwrap();
        for (name, role) in [
            ("老陈", Role::Narrator),
            ("美咲", main_role("高中生，喜欢拍照。")),
            ("健二", main_role("美咲的同学，田径部。")),
            ("店员", Role::Minor),
        ] {
            listening_add_cast(self.state(), WORLD.into(), name.into(), role)
                .await
                .unwrap();
        }
        listening_import_script(self.state(), WORLD.into(), SAMPLE_SCRIPT.into())
            .await
            .unwrap()
    }

    pub fn data_dir(&self) -> PathBuf {
        self.dir.path().join("data")
    }

    pub fn handle(&self) -> AppHandle<MockRuntime> {
        self.app.handle().clone()
    }

    /// Gives ai one machine on this computer whose "python" never answers,
    /// so a synthesis that reaches it runs until it is cancelled.
    pub fn with_hanging_machine(&self) {
        let python = self.dir.path().join("python");
        std::fs::write(&python, "#!/bin/sh\nexec sleep 30\n").unwrap();
        std::fs::set_permissions(&python, std::fs::Permissions::from_mode(0o755)).unwrap();
        self.state()
            .ai
            .update_settings(hanging_machine(&python))
            .unwrap();
    }

    /// Gives ai the GPU machines of ai-invoker's test settings: the
    /// `InvokerSettings` JSON that `NOOBENKYO_INVOKER_TEST_SETTINGS` points at,
    /// whose machines have the two models picked here.
    pub fn with_gpu_machines(&self) {
        let variable = "NOOBENKYO_INVOKER_TEST_SETTINGS";
        let path = std::env::var(variable).unwrap_or_else(|_| panic!("{variable} is not set"));
        let text = std::fs::read_to_string(path).unwrap();
        let invoker: InvokerSettings = serde_json::from_str(&text).unwrap();
        self.state()
            .ai
            .update_settings(AiSettings {
                invoker,
                models: ModelChoice {
                    speak: Some("qwen3-tts-1.7b-base".into()),
                    design_voice: Some("qwen3-tts-1.7b-voicedesign".into()),
                },
            })
            .unwrap();
    }

    /// Every ai-task event sent from now on.
    pub fn task_events(&self) -> TaskEvents {
        let (progress_sender, progress) = unbounded_channel();
        AiTaskProgress::listen(&self.app, move |event| {
            let _ = progress_sender.send(event.payload);
        });
        let (finished_sender, finished) = unbounded_channel();
        AiTaskFinished::listen(&self.app, move |event| {
            let _ = finished_sender.send(event.payload);
        });
        TaskEvents { progress, finished }
    }

    /// Leaves an item in the source's inbox folder as the command line
    /// does: the JSON of one inbox file, written whole under the item's name.
    pub fn deliver(&self, source: InboxSource, name: &str, json: &str) {
        let folder = self.data_dir().join(match source {
            InboxSource::Agent => "inbox-agent",
            InboxSource::Ai => "inbox-ai",
        });
        std::fs::create_dir_all(&folder).unwrap();
        std::fs::write(folder.join(format!("{name}.json")), json).unwrap();
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

pub struct TaskEvents {
    progress: UnboundedReceiver<AiTaskProgress>,
    finished: UnboundedReceiver<AiTaskFinished>,
}

impl TaskEvents {
    /// The next progress event, within 10 seconds.
    pub async fn progress(&mut self) -> AiTaskProgress {
        self.progress_within(10).await
    }

    pub async fn progress_within(&mut self, seconds: u64) -> AiTaskProgress {
        within(seconds, self.progress.recv()).await
    }

    /// The next end, within 10 seconds.
    pub async fn finished(&mut self) -> AiTaskFinished {
        self.finished_within(10).await
    }

    pub async fn finished_within(&mut self, seconds: u64) -> AiTaskFinished {
        within(seconds, self.finished.recv()).await
    }
}

async fn within<T>(seconds: u64, receiving: impl Future<Output = Option<T>>) -> T {
    tokio::time::timeout(Duration::from_secs(seconds), receiving)
        .await
        .unwrap_or_else(|_| panic!("no event within {seconds} seconds"))
        .expect("the app stopped sending events")
}

fn hanging_machine(python: &Path) -> AiSettings {
    AiSettings {
        invoker: InvokerSettings {
            machines: vec![Machine {
                name: "here".into(),
                ssh: None,
                command_layers: vec![],
                runtimes: vec![Runtime::QwenTts {
                    python: python.to_str().unwrap().into(),
                    models: vec![QwenTtsModel {
                        name: "qwen3-tts-1.7b-base".into(),
                        op: TtsOp::TtsClone,
                        path: "/nowhere/base".into(),
                    }],
                }],
                probe: None,
            }],
            ..InvokerSettings::default()
        },
        models: ModelChoice {
            speak: Some("qwen3-tts-1.7b-base".into()),
            design_voice: None,
        },
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
