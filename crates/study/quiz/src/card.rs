use serde::Serialize;

use crate::ComprehensiveVisibility;

/// The face of a comprehensive question.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct Card {
    pub writing: String,
    pub kana: String,
    pub glosses: Vec<String>,
    pub visible: ComprehensiveVisibility,
}
