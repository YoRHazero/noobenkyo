use serde::{Deserialize, Serialize};
use tauri_specta::Event;

use crate::tasks::TaskId;

/// How far a task has got: `done` of `total` items. Sent once with `done`
/// 0 when the total is known, then after every item.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type, Event)]
pub struct AiTaskProgress {
    pub task: TaskId,
    pub done: u32,
    pub total: u32,
}
