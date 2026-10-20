use dictionary::Fragment;
use serde::Serialize;
use study::{Quiz, QuizView};

use crate::command_error::CommandError;
use crate::steps::render::render;

/// A quiz: study's view and the dictionary's rendering of its sentence.
#[derive(Debug, Serialize, specta::Type)]
pub struct QuizPayload {
    pub view: QuizView,
    /// `None` for comprehensive questions, which show no sentence.
    pub sentence: Option<Vec<Fragment>>,
}

impl QuizPayload {
    pub fn render(quiz: Quiz) -> Result<Self, CommandError> {
        Ok(Self {
            view: quiz.view,
            sentence: quiz.sentence.map(render).transpose()?,
        })
    }
}
