use serde::Serialize;

use crate::{Judgement, SentenceToRender};

/// What is revealed after submitting: what the frontend shows, and the
/// sentence for core to render in full.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reference {
    pub view: ReferenceView,
    pub sentence: SentenceToRender,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct ReferenceView {
    /// `None` for comprehensive questions, which take no input.
    pub judgement: Option<Judgement>,
    pub input: String,
    pub writing: String,
    pub kana: String,
    pub glosses: Vec<String>,
    pub translation: Option<String>,
}
