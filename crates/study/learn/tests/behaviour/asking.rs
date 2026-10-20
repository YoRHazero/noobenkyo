//! Choosing the next sense and making its quiz.

use study_learn::LearnError;
use study_quiz::{AnswerError, Content, Next, QuestionType, Rating};

use crate::support::{answer, content, key, open};

#[test]
fn asks_the_first_type_and_repeats_the_quiz_in_progress() {
    let dir = tempfile::tempdir().unwrap();
    let mut learn = open(dir.path(), 1);
    assert_eq!(learn.next(), Next::Empty);
    learn.fill(vec![key("諦める")]).unwrap();
    assert_eq!(learn.next(), Next::NeedContent(key("諦める")));
    let quiz = learn.make_quiz(Some(content("諦める"))).unwrap().unwrap();
    assert_eq!(quiz.view.question_type, QuestionType::Sentence);
    assert_eq!((quiz.view.remaining, quiz.view.steps), (4, 4));
    assert_eq!(learn.next(), Next::Pending(quiz));
}

#[test]
fn a_sense_gone_or_without_sentences_leaves() {
    let dir = tempfile::tempdir().unwrap();
    let mut learn = open(dir.path(), 2);
    learn.fill(vec![key("a"), key("b")]).unwrap();
    learn.next();
    assert_eq!(learn.make_quiz(None).unwrap(), None);
    let Next::NeedContent(left) = learn.next() else {
        panic!("one sense should be left");
    };
    let bare = Content {
        sentences: vec![],
        ..content(&left.writing)
    };
    assert_eq!(learn.make_quiz(Some(bare)).unwrap(), None);
    assert_eq!(learn.next(), Next::Empty);
    assert_eq!(open(dir.path(), 2).next(), Next::Empty);
}

#[test]
fn making_a_quiz_needs_a_chosen_sense() {
    let dir = tempfile::tempdir().unwrap();
    let mut learn = open(dir.path(), 1);
    let error = learn.make_quiz(Some(content("a"))).unwrap_err();
    assert!(matches!(
        error,
        LearnError::Answer(AnswerError::NothingChosen)
    ));
}

#[test]
fn does_not_ask_the_last_k_again() {
    let dir = tempfile::tempdir().unwrap();
    let mut learn = open(dir.path(), 3);
    learn.fill(vec![key("a"), key("b"), key("c")]).unwrap();
    let mut asked: Vec<String> = Vec::new();
    for _ in 0..30 {
        let Next::NeedContent(key) = learn.next() else {
            panic!("expected a sense to ask");
        };
        assert!(!asked.iter().rev().take(2).any(|w| *w == key.writing));
        learn.make_quiz(Some(content(&key.writing))).unwrap();
        learn.submit_answer(String::new()).unwrap();
        learn.confirm_answer(Rating::Vague).unwrap();
        asked.push(key.writing);
    }
}

#[test]
fn a_rated_sense_keeps_its_steps() {
    let dir = tempfile::tempdir().unwrap();
    let mut learn = open(dir.path(), 1);
    learn.fill(vec![key("諦める")]).unwrap();
    assert_eq!(answer(&mut learn, Rating::Known), None);
    let Next::NeedContent(_) = learn.next() else {
        panic!("the sense should still be in the zone");
    };
    let quiz = learn.make_quiz(Some(content("諦める"))).unwrap().unwrap();
    assert_eq!(quiz.view.remaining, 3);
    assert_eq!(quiz.view.question_type, QuestionType::Translation);
}
