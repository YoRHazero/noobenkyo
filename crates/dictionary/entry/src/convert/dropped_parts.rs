use super::sentences::DroppedSentence;

/// Parts of an entry that are dropped while the rest of it is kept.
#[derive(Debug, Default)]
pub struct DroppedParts {
    /// The origin's language, written without an origin.
    pub origin_language: Option<String>,
    pub sentences: Vec<DroppedSentence>,
}
