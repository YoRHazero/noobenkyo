//! Confirming, skipping and reporting, and what is written back.

use study_quiz::{AnswerError, Next, Rating, WriteBack};
use study_review::{ReviewService, ReviewSettings};

use crate::support::{answer, ask, candidate, key, start};

fn one_sense() -> ReviewService {
    start(vec![candidate("a", 3, 30)], None, ReviewSettings::default())
}

fn write(stage: u32, passes: u32, lapses: u32) -> Option<WriteBack> {
    Some(WriteBack {
        key: key("a"),
        stage,
        passes,
        lapses,
    })
}

#[test]
fn known_from_the_pool_is_full_at_once() {
    let mut review = one_sense();
    assert_eq!(answer(&mut review, Rating::Known), write(4, 3, 1));
    assert_eq!(review.next(), Next::Empty);
}

#[test]
fn unknown_writes_at_once_and_starts_the_order_over_in_the_queue() {
    let mut review = one_sense();
    assert_eq!(answer(&mut review, Rating::Unknown), write(0, 2, 2));
    // "a" has no kanji: three types, all to go again.
    assert_eq!(ask(&mut review).view.remaining, 3);
}

#[test]
fn vague_lowers_the_stage_and_known_short_of_full_writes_nothing() {
    let mut review = one_sense();
    assert_eq!(answer(&mut review, Rating::Vague), write(2, 2, 1));
    assert_eq!(ask(&mut review).view.remaining, 2);
    review.submit_answer(String::new()).unwrap();
    assert_eq!(review.confirm_answer(Rating::Known).unwrap(), None);
    assert_eq!(ask(&mut review).view.remaining, 1);
}

#[test]
fn confirming_needs_a_submitted_answer() {
    let mut review = one_sense();
    ask(&mut review);
    assert_eq!(
        review.confirm_answer(Rating::Known),
        Err(AnswerError::NotSubmitted)
    );
}

#[test]
fn report_keeps_the_sense_and_skip_masters_it() {
    let mut review = one_sense();
    ask(&mut review);
    let bad = review.report_sentence().unwrap();
    assert_eq!(bad.marked, "[[a]]だ");
    assert_eq!(review.skip(), Err(AnswerError::NoQuiz));
    assert_eq!(review.next(), Next::NeedContent(key("a")));
    ask(&mut review);
    assert_eq!(review.skip().unwrap().stage, 7);
    assert_eq!(review.next(), Next::Empty);
}
