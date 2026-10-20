//! Submitting, confirming, skipping and reporting.

use study_learn::LearnError;
use study_quiz::{AnswerError, Judgement, Next, Rating, WriteBack};

use crate::support::{answer, ask, key, open};

#[test]
fn confirming_needs_a_submitted_answer() {
    let dir = tempfile::tempdir().unwrap();
    let mut learn = open(dir.path(), 1);
    learn.fill(vec![key("諦める")]).unwrap();
    ask(&mut learn);
    let error = learn.confirm_answer(Rating::Known).unwrap_err();
    assert!(matches!(
        error,
        LearnError::Answer(AnswerError::NotSubmitted)
    ));
    let reference = learn.submit_answer("放弃".into()).unwrap();
    assert_eq!(reference.view.judgement, Some(Judgement::Unsure));
    assert_eq!(learn.confirm_answer(Rating::Known).unwrap(), None);
}

#[test]
fn full_leaves_and_hands_back_one_more_pass() {
    let dir = tempfile::tempdir().unwrap();
    let mut learn = open(dir.path(), 1);
    learn.fill(vec![key("諦める")]).unwrap();
    for _ in 0..3 {
        assert_eq!(answer(&mut learn, Rating::Known), None);
    }
    assert_eq!(
        answer(&mut learn, Rating::Known),
        Some(WriteBack {
            key: key("諦める"),
            stage: 1,
            passes: 1,
            lapses: 1
        })
    );
    assert_eq!(learn.next(), Next::Empty);
    assert_eq!(open(dir.path(), 1).next(), Next::Empty);
}

#[test]
fn unknown_starts_the_order_over() {
    let dir = tempfile::tempdir().unwrap();
    let mut learn = open(dir.path(), 1);
    learn.fill(vec![key("諦める")]).unwrap();
    answer(&mut learn, Rating::Known);
    answer(&mut learn, Rating::Known);
    assert_eq!(answer(&mut learn, Rating::Unknown), None);
    assert_eq!(ask(&mut learn).view.remaining, 4);
}

#[test]
fn skip_masters_the_sense() {
    let dir = tempfile::tempdir().unwrap();
    let mut learn = open(dir.path(), 1);
    learn.fill(vec![key("諦める")]).unwrap();
    ask(&mut learn);
    let write = learn.skip().unwrap();
    assert_eq!((write.stage, write.passes), (7, 0));
    assert_eq!(learn.next(), Next::Empty);
    assert!(matches!(
        learn.skip().unwrap_err(),
        LearnError::Answer(AnswerError::NoQuiz)
    ));
}

#[test]
fn report_drops_the_quiz_but_keeps_the_steps() {
    let dir = tempfile::tempdir().unwrap();
    let mut learn = open(dir.path(), 1);
    learn.fill(vec![key("諦める")]).unwrap();
    answer(&mut learn, Rating::Known);
    ask(&mut learn);
    learn.submit_answer(String::new()).unwrap();
    let bad = learn.report_sentence().unwrap();
    assert_eq!(bad.marked, "[[諦める]]だ");
    assert_eq!(learn.next(), Next::NeedContent(key("諦める")));
    assert_eq!(ask(&mut learn).view.remaining, 3);
}
