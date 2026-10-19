//! End-to-end flows over real services in a temporary directory: import,
//! learning, settling and review.

use dictionary::{DictionaryService, ImportOptions};
use study::{AnswerInput, QuestionType, Rating, ReviewMode, StudyService, Zone};

use super::question::current_question;
use super::settle::settle_and_remove;
use super::start_group::start_group;
use crate::app_state::AppState;

const SAMPLE: &str = include_str!("../../../../samples/basic.jsonl");

async fn imported_state(dir: &std::path::Path) -> AppState {
    let state = AppState::open(dir).await.unwrap();
    let report = state
        .dictionary
        .import(SAMPLE, ImportOptions::default())
        .await
        .unwrap();
    assert!(report.problems.is_empty(), "{:?}", report.problems);
    assert!(report.ignored_fields.is_empty());
    assert_eq!(report.skipped_katakana, 2);
    state
}

/// Answers the zone's current question with a rating, as the commands do.
async fn answer(
    dictionary: &DictionaryService,
    study: &mut StudyService,
    zone: Zone,
    rating: Rating,
) -> bool {
    let Some(question) = current_question(dictionary, study, zone).await.unwrap() else {
        return false;
    };
    let input = match question.view.answer_input {
        AnswerInput::None => String::new(),
        _ => "x".to_owned(),
    };
    study.submit(zone, input).unwrap();
    let outcome = study.rate(zone, rating).unwrap();
    if let Some(settlement) = outcome.settlement {
        settle_and_remove(dictionary, study, zone, settlement)
            .await
            .unwrap();
    }
    true
}

#[tokio::test]
async fn learning_fills_asks_in_order_and_settles_full_senses() {
    let dir = tempfile::tempdir().unwrap();
    let state = imported_state(dir.path()).await;
    let mut study = state.study.lock().await;

    let first = current_question(&state.dictionary, &mut study, Zone::Learning)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(first.view.question_type, QuestionType::Sentence);
    assert!(first.fragments.is_some());
    assert_eq!(study.status(Zone::Learning).items, 9);

    let mut answered = 0;
    while answer(&state.dictionary, &mut study, Zone::Learning, Rating::Known).await {
        answered += 1;
        assert!(answered < 100, "learning never empties");
    }
    // 7 kanji words take 4 steps, きれい takes 3, 消しゴム has kanji: 8 × 4 + 3.
    assert_eq!(answered, 35);
    assert_eq!(study.status(Zone::Learning).new_today, 9);

    let candidates = state.dictionary.candidates().await.unwrap();
    assert_eq!(candidates.len(), 9);
    assert!(
        candidates
            .iter()
            .all(|c| c.progress.passes == 1 && c.progress.stage == 1)
    );
}

#[tokio::test]
async fn review_counts_lapses_and_settles() {
    let dir = tempfile::tempdir().unwrap();
    let state = imported_state(dir.path()).await;
    let mut study = state.study.lock().await;
    while answer(&state.dictionary, &mut study, Zone::Learning, Rating::Known).await {}

    let question = current_question(&state.dictionary, &mut study, Zone::Review)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(question.view.question_type, QuestionType::Comprehensive);
    assert_eq!(study.status(Zone::Review).items, 9);

    assert!(answer(&state.dictionary, &mut study, Zone::Review, Rating::Unknown).await);
    while answer(&state.dictionary, &mut study, Zone::Review, Rating::Known).await {
        if study.status(Zone::Review).items < 9 {
            break;
        }
    }
    let settled = state
        .dictionary
        .candidates()
        .await
        .unwrap()
        .into_iter()
        .filter(|c| c.progress.passes == 2)
        .count();
    assert_eq!(settled, 1);
}

#[tokio::test]
async fn fixed_group_starts_on_request_and_switching_settles() {
    let dir = tempfile::tempdir().unwrap();
    let state = imported_state(dir.path()).await;
    let mut study = state.study.lock().await;
    while answer(&state.dictionary, &mut study, Zone::Learning, Rating::Known).await {}

    current_question(&state.dictionary, &mut study, Zone::Review)
        .await
        .unwrap();
    assert_eq!(study.status(Zone::Review).items, 9);
    for settlement in study.review_settlements() {
        super::settle::write_back(&state.dictionary, &settlement)
            .await
            .unwrap();
    }
    study.set_review_mode(ReviewMode::FixedGroup).unwrap();
    assert!(
        current_question(&state.dictionary, &mut study, Zone::Review)
            .await
            .unwrap()
            .is_none()
    );
    assert!(study.status(Zone::Review).group_finished);

    start_group(&state.dictionary, &mut study).await.unwrap();
    assert_eq!(study.status(Zone::Review).items, 9);
}

#[tokio::test]
async fn reported_sentence_is_deleted_and_sense_leaves_without_settling() {
    let dir = tempfile::tempdir().unwrap();
    let state = imported_state(dir.path()).await;
    let mut study = state.study.lock().await;
    current_question(&state.dictionary, &mut study, Zone::Learning)
        .await
        .unwrap();

    let reported = study.report_sentence(Zone::Learning).unwrap();
    let key = crate::convert::sense_key::to_dictionary(reported.key.clone());
    state
        .dictionary
        .remove_sentence(&key, &reported.marked)
        .await
        .unwrap();
    study.remove(Zone::Learning, &reported.key).unwrap();
    assert_eq!(study.status(Zone::Learning).items, 8);
}
