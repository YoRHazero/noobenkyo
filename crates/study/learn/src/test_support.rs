//! Test data shared by the rule tests.

use rand::SeedableRng;
use rand::rngs::StdRng;
use study_quiz::SenseKey;

pub fn rng() -> StdRng {
    StdRng::seed_from_u64(7)
}

pub fn key(writing: &str) -> SenseKey {
    SenseKey {
        writing: writing.into(),
        origin: String::new(),
        kana: "かな".into(),
        primary_gloss: "释义".into(),
    }
}
