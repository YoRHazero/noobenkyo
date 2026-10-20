use crate::{Progress, SenseKey};

/// Progress to write back to a sense: exactly what the dictionary's
/// `update_progress` takes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WriteBack {
    pub key: SenseKey,
    pub stage: u32,
    pub passes: u32,
    pub lapses: u32,
}

impl WriteBack {
    pub fn new(key: SenseKey, progress: Progress) -> Self {
        Self {
            key,
            stage: progress.stage,
            passes: progress.passes,
            lapses: progress.lapses,
        }
    }
}
