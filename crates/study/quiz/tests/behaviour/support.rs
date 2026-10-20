//! Test data shared by the behaviour tests.

use rand::SeedableRng;
use rand::rngs::StdRng;
use study_quiz::{Content, Progress, SenseKey, Sentence};

pub fn rng() -> StdRng {
    StdRng::seed_from_u64(7)
}

pub fn key(writing: &str) -> SenseKey {
    SenseKey {
        writing: writing.into(),
        origin: String::new(),
        kana: "あきらめる".into(),
        primary_gloss: "放弃".into(),
    }
}

/// Content with the given marked sentences, at stage 2 with 3 passes.
pub fn content(sentences: &[&str]) -> Content {
    Content {
        glosses: vec!["放弃".into(), "死心".into()],
        sentences: sentences
            .iter()
            .map(|marked| Sentence {
                marked: (*marked).into(),
                translation: Some("译文".into()),
            })
            .collect(),
        progress: Progress {
            stage: 2,
            passes: 3,
            lapses: 1,
        },
    }
}
