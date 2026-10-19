use serde::{Deserialize, Serialize};

use crate::SentenceContent;

/// Changes to the content of one sense. A field left `None` keeps its
/// current value; a given field replaces it as a whole. The layers above the
/// sense and its progress cannot be changed here.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct SensePatch {
    /// The new glosses, the primary gloss first.
    pub glosses: Option<Vec<String>>,
    pub labels: Option<Vec<String>>,
    pub sentences: Option<Vec<SentenceContent>>,
}
