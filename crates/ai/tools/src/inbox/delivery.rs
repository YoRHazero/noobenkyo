use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// One thing an agent delivered, as it is kept in an inbox file.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Delivery {
    /// Entries, opened in the import dialog with the options for generated lists.
    Entries { text: String },
    /// A script, opened in the script preview.
    Script { target: ScriptTarget, text: String },
}

/// Where a delivered script goes once the user saves it. The frontend gets
/// it as it is and hands it back when writing edits back or saving.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ScriptTarget {
    /// A new episode of the world, numbered by default.
    NewEpisode { world: String },
    /// More lines for this episode.
    Append { episode: Uuid },
}
