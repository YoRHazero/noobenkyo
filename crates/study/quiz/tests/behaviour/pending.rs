//! The quiz in progress: made, submitted, rated, reported.

use study_quiz::{
    AnswerError, Judgement, Pending, QuestionType, QuizSettings, Rating, SentenceDisplay,
};

use crate::support::{content, key, rng};

fn make(remaining: u32, sentences: &[&str]) -> Option<Pending> {
    Pending::make(
        key("諦める"),
        remaining,
        content(sentences),
        &QuizSettings::default(),
        &mut rng(),
    )
}

#[test]
fn needs_a_sentence() {
    assert!(make(4, &[]).is_none());
}

#[test]
fn asks_the_type_counted_from_the_end() {
    let pending = make(4, &["[[{諦|あきら}める]]"]).unwrap();
    assert_eq!(pending.quiz().view.question_type, QuestionType::Sentence);
    let pending = make(9, &["[[{諦|あきら}める]]"]).unwrap();
    assert_eq!(pending.quiz().view.remaining, 4);
}

#[test]
fn rating_needs_a_submitted_answer() {
    let mut pending = make(2, &["[[{諦|あきら}める]]"]).unwrap();
    assert_eq!(pending.rate(Rating::Known), Err(AnswerError::NotSubmitted));
    let reference = pending.submit("アキラメル".into());
    assert_eq!(reference.view.judgement, Some(Judgement::Correct));
    assert_eq!(reference.sentence.display, SentenceDisplay::Full);
    assert_eq!(reference.view.translation.as_deref(), Some("译文"));
    assert_eq!(pending.rate(Rating::Known), Ok(1));
    assert_eq!(pending.rate(Rating::Unknown), Ok(4));
}

#[test]
fn reports_the_sentence_it_shows() {
    let pending = make(1, &["[[{諦|あきら}める]]"]).unwrap();
    assert_eq!(pending.bad_sentence().marked, "[[{諦|あきら}める]]");
    assert_eq!(pending.progress().passes, 3);
}
