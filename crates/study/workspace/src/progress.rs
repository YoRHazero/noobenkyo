use serde::{Deserialize, Serialize};

/// The copy of a sense's progress that a zone works on until settlement.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Progress {
    pub stage: u32,
    pub passes: u32,
    pub lapses: u32,
}
