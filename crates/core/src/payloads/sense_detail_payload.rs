use dictionary::{Fragment, SenseDetail, SentenceDisplay};
use serde::Serialize;

use crate::command_error::CommandError;

/// One sense in full, with each sentence rendered in full. The marked texts
/// stay in `detail`, for editing.
#[derive(Debug, Serialize, specta::Type)]
pub struct SenseDetailPayload {
    pub detail: SenseDetail,
    /// In the order of `detail.sentences`.
    pub sentences: Vec<Vec<Fragment>>,
}

impl SenseDetailPayload {
    pub fn render(detail: SenseDetail) -> Result<Self, CommandError> {
        let sentences = detail
            .sentences
            .iter()
            .map(|sentence| dictionary::render_sentence(&sentence.marked, SentenceDisplay::Full))
            .collect::<Result<_, _>>()?;
        Ok(Self { detail, sentences })
    }
}
