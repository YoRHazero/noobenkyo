//! Choosing between the pool and the queue, and making quizzes.

use study_quiz::{Content, Next, QuestionType, Rating};
use study_review::ReviewSettings;

use crate::support::{answer, candidate, content, key, start};

#[test]
fn asks_the_pool_in_order_with_the_last_type() {
    let mut review = start(
        vec![candidate("old", 0, 1), candidate("older", 0, 5)],
        None,
        ReviewSettings::default(),
    );
    assert_eq!(review.next(), Next::NeedContent(key("older")));
    let quiz = review.make_quiz(Some(content("older"))).unwrap().unwrap();
    assert_eq!(quiz.view.question_type, QuestionType::Comprehensive);
    assert_eq!(review.next(), Next::Pending(quiz));
}

#[test]
fn a_sense_gone_leaves_the_session() {
    let mut review = start(vec![candidate("a", 0, 1)], None, ReviewSettings::default());
    review.next();
    let bare = Content {
        sentences: vec![],
        ..content("a")
    };
    assert_eq!(review.make_quiz(Some(bare)).unwrap(), None);
    assert_eq!(review.next(), Next::Empty);
}

#[test]
fn a_full_queue_is_always_asked_from() {
    let settings = ReviewSettings {
        queue_limit: 1,
        recent_limit: 0,
        ..ReviewSettings::default()
    };
    let mut review = start(
        vec![candidate("q", 0, 5), candidate("p", 0, 1)],
        None,
        settings,
    );
    answer(&mut review, Rating::Vague);
    for _ in 0..20 {
        assert_eq!(review.next(), Next::NeedContent(key("q")));
    }
}

#[test]
fn the_other_side_steps_in_rather_than_repeating() {
    let settings = ReviewSettings {
        queue_limit: 2,
        recent_limit: 1,
        ..ReviewSettings::default()
    };
    let mut review = start(
        vec![candidate("q", 0, 5), candidate("p", 0, 1)],
        None,
        settings,
    );
    answer(&mut review, Rating::Vague);
    // The queue is preferred half the time; q was just asked, so the pool
    // steps in every time.
    for _ in 0..20 {
        assert_eq!(review.next(), Next::NeedContent(key("p")));
    }
}

#[test]
fn the_last_sense_may_repeat() {
    let mut review = start(vec![candidate("q", 0, 5)], None, ReviewSettings::default());
    answer(&mut review, Rating::Unknown);
    for _ in 0..3 {
        assert_eq!(review.next(), Next::NeedContent(key("q")));
        answer(&mut review, Rating::Vague);
    }
}
