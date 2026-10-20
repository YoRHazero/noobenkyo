//! Test data and steps shared by the behaviour tests.

use std::path::Path;

use rand::SeedableRng;
use rand::rngs::StdRng;
use study_learn::{LearnService, LearnSettings};
use study_quiz::{
    Content, Next, Progress, Quiz, QuizSettings, Rating, SenseKey, Sentence, WriteBack,
};

pub fn key(writing: &str) -> SenseKey {
    SenseKey {
        writing: writing.into(),
        origin: String::new(),
        kana: "かな".into(),
        primary_gloss: "释义".into(),
    }
}

/// Content with one sentence whose target is the writing; untouched, one lapse.
pub fn content(writing: &str) -> Content {
    Content {
        glosses: vec!["释义".into()],
        sentences: vec![Sentence {
            marked: format!("[[{writing}]]だ"),
            translation: Some("译文".into()),
        }],
        progress: Progress {
            stage: 0,
            passes: 0,
            lapses: 1,
        },
    }
}

/// A learning zone of the given capacity in `dir`.
pub fn open(dir: &Path, capacity: u32) -> LearnService {
    LearnService::open(
        dir,
        QuizSettings::default(),
        LearnSettings {
            capacity,
            ..LearnSettings::default()
        },
        StdRng::seed_from_u64(7),
    )
}

/// Asks the next sense, as core does: its content comes from the dictionary.
pub fn ask(learn: &mut LearnService) -> Quiz {
    match learn.next() {
        Next::Pending(quiz) => quiz,
        Next::NeedContent(key) => learn
            .make_quiz(Some(content(&key.writing)))
            .unwrap()
            .expect("the sense has a sentence"),
        Next::Empty => panic!("the zone is empty"),
    }
}

/// Asks, submits an empty answer and confirms the rating.
pub fn answer(learn: &mut LearnService, rating: Rating) -> Option<WriteBack> {
    ask(learn);
    learn.submit_answer(String::new()).unwrap();
    learn.confirm_answer(rating).unwrap()
}
