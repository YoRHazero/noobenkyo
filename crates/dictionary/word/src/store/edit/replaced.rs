use serde::Serialize;

use crate::SenseKey;

/// What replacing a sense's content did.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Replaced {
    /// Replaced; the key changes when the primary gloss does.
    Done {
        key: SenseKey,
    },
    /// Nothing was changed, for these reasons (in Chinese).
    Rejected {
        problems: Vec<String>,
    },
    NotFound,
}
