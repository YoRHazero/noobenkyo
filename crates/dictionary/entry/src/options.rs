use serde::{Deserialize, Serialize};

/// Choices made in the import dialog for one import.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct ImportOptions {
    /// Skip entries whose writing is pure katakana.
    pub drop_pure_katakana: bool,
    /// Drop entries that look inflected, reporting each one.
    pub detect_inflections: bool,
    /// Take progress from the text; when false, progress fields are ignored.
    pub keep_progress: bool,
}

impl ImportOptions {
    /// For lists a model or an agent wrote, as opposed to outside word lists.
    /// Katakana words stay, because rows exported for filling in sentences
    /// contain them; inflections are not flagged, because a model cannot fix
    /// a false alarm; progress is ignored, so a model cannot change it.
    pub fn generated() -> Self {
        Self {
            drop_pure_katakana: false,
            detect_inflections: false,
            keep_progress: false,
        }
    }
}

impl Default for ImportOptions {
    fn default() -> Self {
        Self {
            drop_pure_katakana: true,
            detect_inflections: false,
            keep_progress: true,
        }
    }
}
