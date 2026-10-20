//! Test data and steps shared by the behaviour tests.

use chrono::{DateTime, Duration, FixedOffset};
use rand::SeedableRng;
use rand::rngs::StdRng;
use study_quiz::{
    Content, Next, Progress, Quiz, QuizSettings, Rating, SenseKey, Sentence, WriteBack,
};
use study_review::{Candidate, ReviewService, ReviewSettings, SessionOptions};

pub fn now() -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339("2026-10-20T09:00:00+09:00").unwrap()
}

pub fn key(writing: &str) -> SenseKey {
    SenseKey {
        writing: writing.into(),
        origin: String::new(),
        kana: "かな".into(),
        primary_gloss: "释义".into(),
    }
}

/// A learned sense at the given stage, last modified `days_ago` days before `now()`.
pub fn candidate(writing: &str, stage: u32, days_ago: i64) -> Candidate {
    Candidate {
        key: key(writing),
        stage,
        passes: 1,
        lapses: 0,
        modified_at: now() - Duration::days(days_ago),
    }
}

/// Content with one sentence whose target is the writing, at stage 3.
pub fn content(writing: &str) -> Content {
    Content {
        glosses: vec!["释义".into()],
        sentences: vec![Sentence {
            marked: format!("[[{writing}]]だ"),
            translation: None,
        }],
        progress: Progress {
            stage: 3,
            passes: 2,
            lapses: 1,
        },
    }
}

/// A session over the candidates, most overdue first; `size` `None` is endless.
pub fn start(
    candidates: Vec<Candidate>,
    size: Option<u32>,
    settings: ReviewSettings,
) -> ReviewService {
    ReviewService::start(
        candidates,
        SessionOptions {
            size,
            ..SessionOptions::default()
        },
        QuizSettings::default(),
        settings,
        now(),
        StdRng::seed_from_u64(7),
    )
}

/// Asks the next sense, as core does: its content comes from the dictionary.
pub fn ask(review: &mut ReviewService) -> Quiz {
    match review.next() {
        Next::Pending(quiz) => quiz,
        Next::NeedContent(key) => review
            .make_quiz(Some(content(&key.writing)))
            .unwrap()
            .expect("the sense has a sentence"),
        Next::Empty => panic!("the session is over"),
    }
}

/// Asks, submits an empty answer and confirms the rating.
pub fn answer(review: &mut ReviewService, rating: Rating) -> Option<WriteBack> {
    ask(review);
    review.submit_answer(String::new()).unwrap();
    review.confirm_answer(rating).unwrap()
}
