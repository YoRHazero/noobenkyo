//! The review line: a session over learned senses, from start to end.

use dictionary::StageCount;
use noobenkyo_lib::commands::review::answering::{
    review_confirm_answer, review_report_sentence, review_skip, review_submit_answer,
};
use noobenkyo_lib::commands::review::asking::review_next;
use noobenkyo_lib::commands::review::session::{review_end, review_start};
use noobenkyo_lib::commands::stats::dictionary::dictionary_stats;
use study::{Rating, SessionOptions};

use crate::support::{TestApp, learn_everything, review_once};

const NO_SESSION: &str = "当前没有进行中的复习";

/// The sample, every sense learned to stage 1, and a session started.
async fn reviewing() -> TestApp {
    let app = TestApp::with_sample().await;
    learn_everything(&app).await;
    review_start(app.state(), SessionOptions::default())
        .await
        .unwrap();
    app
}

#[tokio::test]
async fn without_a_session_only_starting_works() {
    let app = TestApp::with_sample().await;
    assert_eq!(
        review_next(app.state()).await.unwrap_err().message,
        NO_SESSION
    );
    let errors = [
        review_submit_answer(app.state(), String::new())
            .await
            .map(|_| ())
            .unwrap_err(),
        review_confirm_answer(app.state(), Rating::Known)
            .await
            .unwrap_err(),
        review_skip(app.state()).await.unwrap_err(),
        review_report_sentence(app.state()).await.unwrap_err(),
    ];
    assert!(errors.iter().all(|error| error.message == NO_SESSION));
}

#[tokio::test]
async fn a_session_asks_every_learned_sense_until_the_group_is_done() {
    let app = reviewing().await;
    let mut quizzes = 0;
    while review_once(&app, Rating::Known).await {
        quizzes += 1;
        assert!(quizzes < 100, "the session never ends");
    }
    // Start steps 1: known once is full.
    assert_eq!(quizzes, 9);
    assert!(review_next(app.state()).await.unwrap().is_none());
    let stats = dictionary_stats(app.state()).await.unwrap();
    assert_eq!(
        stats.stages,
        [StageCount {
            stage: 2,
            senses: 9
        }]
    );
}

#[tokio::test]
async fn unknown_is_written_back_at_once() {
    let app = reviewing().await;
    assert!(review_once(&app, Rating::Unknown).await);
    let stats = dictionary_stats(app.state()).await.unwrap();
    assert_eq!(
        stats.stages,
        [
            StageCount {
                stage: 0,
                senses: 1
            },
            StageCount {
                stage: 1,
                senses: 8
            }
        ]
    );
}

#[tokio::test]
async fn ending_drops_the_session() {
    let app = reviewing().await;
    assert!(review_next(app.state()).await.unwrap().is_some());
    review_end(app.state()).await.unwrap();
    assert_eq!(
        review_next(app.state()).await.unwrap_err().message,
        NO_SESSION
    );
}

#[tokio::test]
async fn invalid_options_start_nothing() {
    let app = TestApp::with_sample().await;
    let options = SessionOptions {
        start_steps: 0,
        ..SessionOptions::default()
    };
    assert!(review_start(app.state(), options).await.is_err());
    assert_eq!(
        review_next(app.state()).await.unwrap_err().message,
        NO_SESSION
    );
}

#[tokio::test]
async fn nothing_learned_is_a_session_done_at_once() {
    let app = TestApp::with_sample().await;
    review_start(app.state(), SessionOptions::default())
        .await
        .unwrap();
    assert!(review_next(app.state()).await.unwrap().is_none());
}
