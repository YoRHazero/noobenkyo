use crate::SenseKey;

/// A sentence reported as wrong: exactly what the dictionary's
/// `delete_sentence` takes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BadSentence {
    pub key: SenseKey,
    pub marked: String,
}
