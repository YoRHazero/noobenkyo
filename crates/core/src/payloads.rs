//! What the question commands return: a feature's view plus the rendered sentence.

use serde::Serialize;

#[derive(Debug, Serialize, specta::Type)]
pub struct QuestionPayload {
    pub view: study::QuestionView,
    pub fragments: Option<Vec<dictionary::Fragment>>,
}

#[derive(Debug, Serialize, specta::Type)]
pub struct RevealPayload {
    pub view: study::RevealView,
    pub fragments: Option<Vec<dictionary::Fragment>>,
}
