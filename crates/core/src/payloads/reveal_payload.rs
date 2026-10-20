use dictionary::Fragment;
use serde::Serialize;
use study::{Reference, ReferenceView};

use crate::command_error::CommandError;
use crate::steps::render::render;

/// What is revealed after submitting: study's view and the sentence in full.
#[derive(Debug, Serialize, specta::Type)]
pub struct RevealPayload {
    pub view: ReferenceView,
    pub sentence: Vec<Fragment>,
}

impl RevealPayload {
    pub fn render(reference: Reference) -> Result<Self, CommandError> {
        Ok(Self {
            view: reference.view,
            sentence: render(reference.sentence)?,
        })
    }
}
