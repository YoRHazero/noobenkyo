//! The learning line: topping the zone up, asking, answering and writing back.

use dictionary::{ContentFilter, Fragment, SenseOrder, StageCount};
use noobenkyo_core::commands::dictionary::delete::dictionary_delete_sense;
use noobenkyo_core::commands::dictionary::query::{
    dictionary_filter_senses, dictionary_sense_detail,
};
use noobenkyo_core::commands::learn::answering::{
    learn_confirm_answer, learn_report_sentence, learn_skip, learn_submit_answer,
};
use noobenkyo_core::commands::learn::asking::learn_next;
use noobenkyo_core::commands::stats::dictionary::dictionary_stats;
use study::{QuestionType, Rating};

use crate::support::{TestApp, learn_everything, learn_once, revealed_key};

#[tokio::test]
async fn an_empty_dictionary_has_nothing_to_learn() {
    let app = TestApp::fresh().await;
    assert!(learn_next(app.state()).await.unwrap().is_none());
}

#[tokio::test]
async fn the_first_quiz_is_a_sentence_quiz_asked_until_answered() {
    let app = TestApp::with_sample().await;
    let quiz = learn_next(app.state()).await.unwrap().unwrap();
    assert_eq!(quiz.view.question_type, QuestionType::Sentence);
    let sentence = quiz.sentence.unwrap();
    assert!(
        sentence
            .iter()
            .any(|fragment| matches!(fragment, Fragment::Target { .. }))
    );

    let again = learn_next(app.state()).await.unwrap().unwrap();
    assert_eq!(again.view, quiz.view);
    assert_eq!(again.sentence.unwrap(), sentence);
}

#[tokio::test]
async fn learning_everything_writes_each_sense_back_once() {
    let app = TestApp::with_sample().await;
    // Eight words with kanji take four steps each, きれい takes three.
    assert_eq!(learn_everything(&app).await, 35);
    let stats = dictionary_stats(app.state()).await.unwrap();
    assert_eq!(stats.untouched, 0);
    assert_eq!(
        stats.stages,
        [StageCount {
            stage: 1,
            senses: 9
        }]
    );
}

#[tokio::test]
async fn the_reveal_shows_the_sentence_in_full() {
    let app = TestApp::with_sample().await;
    learn_next(app.state()).await.unwrap();
    let reveal = learn_submit_answer(app.state(), "放弃".into())
        .await
        .unwrap();
    let target = reveal
        .sentence
        .iter()
        .find(|fragment| matches!(fragment, Fragment::Target { .. }));
    assert!(target.is_some());
    assert!(!reveal.view.glosses.is_empty());
}

#[tokio::test]
async fn confirming_before_submitting_is_an_error() {
    let app = TestApp::with_sample().await;
    learn_next(app.state()).await.unwrap();
    let error = learn_confirm_answer(app.state(), Rating::Known)
        .await
        .unwrap_err();
    assert_eq!(error.message, "还没有提交答案");
}

#[tokio::test]
async fn skipping_masters_the_sense() {
    let app = TestApp::with_sample().await;
    learn_next(app.state()).await.unwrap();
    learn_skip(app.state()).await.unwrap();
    let stats = dictionary_stats(app.state()).await.unwrap();
    assert_eq!(stats.untouched, 8);
    assert_eq!(
        stats.stages,
        [StageCount {
            stage: 7,
            senses: 1
        }]
    );
}

#[tokio::test]
async fn reporting_deletes_the_sentence_and_keeps_the_sense() {
    let app = TestApp::with_sample().await;
    learn_next(app.state()).await.unwrap();
    let reveal = learn_submit_answer(app.state(), String::new())
        .await
        .unwrap();
    let key = revealed_key(&reveal);
    let before = dictionary_sense_detail(app.state(), key.clone())
        .await
        .unwrap()
        .unwrap();

    learn_report_sentence(app.state()).await.unwrap();
    let after = dictionary_sense_detail(app.state(), key)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(after.sentences.len(), before.sentences.len() - 1);
    assert!(learn_next(app.state()).await.unwrap().is_some());
}

#[tokio::test]
async fn a_sense_deleted_during_its_quiz_is_not_written_back() {
    let app = TestApp::with_sample().await;
    learn_next(app.state()).await.unwrap();
    let reveal = learn_submit_answer(app.state(), String::new())
        .await
        .unwrap();
    assert!(
        dictionary_delete_sense(app.state(), revealed_key(&reveal))
            .await
            .unwrap()
    );
    learn_skip(app.state()).await.unwrap();
    assert!(
        dictionary_stats(app.state())
            .await
            .unwrap()
            .stages
            .is_empty()
    );
}

#[tokio::test]
async fn senses_deleted_from_the_dictionary_leave_the_zone() {
    let app = TestApp::with_sample().await;
    assert!(learn_once(&app, Rating::Known).await);
    let senses = dictionary_filter_senses(
        app.state(),
        ContentFilter::default(),
        vec![],
        SenseOrder::Creation,
        None,
    )
    .await
    .unwrap();
    for sense in senses {
        dictionary_delete_sense(app.state(), sense.key)
            .await
            .unwrap();
    }
    assert!(learn_next(app.state()).await.unwrap().is_none());
}
