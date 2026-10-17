use serde::{Deserialize, Serialize};

/// Which parts of a comprehensive question are shown before answering.
/// Hidden parts are covered until revealed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(default)]
pub struct ComprehensiveVisibility {
    pub glosses: bool,
    pub writing: bool,
    pub kana: bool,
}

impl Default for ComprehensiveVisibility {
    fn default() -> Self {
        Self {
            glosses: false,
            writing: true,
            kana: true,
        }
    }
}
