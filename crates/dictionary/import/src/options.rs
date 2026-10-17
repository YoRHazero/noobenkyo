use serde::{Deserialize, Serialize};

/// Choices made in the import dialog for one import.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct ImportOptions {
    /// Skip entries whose writing is pure katakana.
    pub drop_pure_katakana: bool,
    /// Put entries that look inflected into the warning list instead of importing them.
    pub detect_inflections: bool,
    /// Take progress from the file; when false, progress fields are ignored.
    pub keep_progress: bool,
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
