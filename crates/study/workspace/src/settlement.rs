use crate::{Progress, SenseKey};

/// Progress to write back to the dictionary when a sense leaves a zone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settlement {
    pub key: SenseKey,
    pub progress: Progress,
}
