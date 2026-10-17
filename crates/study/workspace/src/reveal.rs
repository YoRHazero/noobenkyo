use serde::Serialize;
use study_answer::Rating;

use crate::SentenceToRender;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reveal {
    pub view: RevealView,
    pub sentence: Option<SentenceToRender>,
}

/// The answer shown after submitting.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct RevealView {
    pub writing: String,
    pub kana: String,
    pub glosses: Vec<String>,
    pub translation: Option<String>,
    /// The kana the question expected, if it can be checked.
    pub expected: Option<String>,
    pub input: String,
    pub prediction: Option<Rating>,
}
