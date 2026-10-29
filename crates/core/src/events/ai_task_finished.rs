use serde::{Deserialize, Serialize};
use tauri_specta::Event;

use crate::tasks::TaskId;

/// A task has ended and left the table. What it stored before the end stays
/// stored, whatever the outcome.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type, Event)]
pub struct AiTaskFinished {
    pub task: TaskId,
    pub outcome: TaskOutcome,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TaskOutcome {
    Completed,
    Cancelled,
    /// With the message of what went wrong, as the feature wrote it.
    Failed {
        message: String,
    },
}
