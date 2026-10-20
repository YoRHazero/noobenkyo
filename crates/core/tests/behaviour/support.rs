//! Test data and steps shared by the behaviour tests.

use dictionary::{ImportReport, SenseKey};
use noobenkyo_lib::app_state::AppState;
use noobenkyo_lib::commands::dictionary::entries::{
    dictionary_default_import_options, dictionary_import,
};
use noobenkyo_lib::commands::learn::answering::{learn_confirm_answer, learn_submit_answer};
use noobenkyo_lib::commands::learn::asking::learn_next;
use noobenkyo_lib::commands::review::answering::{review_confirm_answer, review_submit_answer};
use noobenkyo_lib::commands::review::asking::review_next;
use noobenkyo_lib::payloads::RevealPayload;
use study::Rating;
use tauri::test::{MockRuntime, mock_app};
use tauri::{App, Manager, State};

const SAMPLE: &str = include_str!("../../../../samples/basic.jsonl");

/// The app over its own temporary data directory.
pub struct TestApp {
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

    /// A file the user could pick in a dialog, holding `text`.
    pub fn user_file(&self, name: &str, text: &str) -> String {
        let path = self.dir.path().join(name);
        std::fs::write(&path, text).unwrap();
        path.display().to_string()
    }

    pub async fn import_sample(&self) -> ImportReport {
        let path = self.user_file("basic.jsonl", SAMPLE);
        dictionary_import(self.state(), path, dictionary_default_import_options())
            .await
            .unwrap()
    }
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
