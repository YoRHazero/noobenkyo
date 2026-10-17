use crate::{Progress, SenseKey};

/// A sense that has at least one sentence and may enter a zone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SenseCandidate {
    pub key: SenseKey,
    pub progress: Progress,
}
