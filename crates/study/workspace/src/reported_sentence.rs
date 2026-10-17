use crate::SenseKey;

/// A sentence the user reported as wrong.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportedSentence {
    pub key: SenseKey,
    pub marked: String,
}
