//! Starting a session: which senses it holds.

use study_quiz::{Next, Rating};
use study_review::ReviewSettings;

use crate::support::{answer, candidate, start};

/// Answers "known" until the session is over; the senses asked, in order.
fn run_through(mut review: study_review::ReviewService) -> Vec<String> {
    let mut asked = Vec::new();
    while let Next::NeedContent(key) = review.next() {
        asked.push(key.writing);
        answer(&mut review, Rating::Known);
    }
    asked
}

#[test]
fn a_group_holds_the_most_overdue_and_leaves_out_the_mastered() {
    let candidates = vec![
        candidate("fresh", 0, 0),
        candidate("overdue", 2, 30),
        candidate("mastered", 7, 300),
        candidate("due", 0, 1),
    ];
    let group = start(candidates.clone(), Some(2), ReviewSettings::default());
    assert_eq!(run_through(group), ["overdue", "due"]);
    let endless = start(candidates, None, ReviewSettings::default());
    assert_eq!(run_through(endless), ["overdue", "due", "fresh"]);
}
