//! Jobs that take minutes. The command that starts one returns a task id at
//! once; the job runs in the background, and its progress and end reach
//! every page as the ai-task events, keyed by that id. One shared command
//! cancels any of them. Closing the app ends them; what they stored stays.

mod run;
pub mod speak_lines;
mod table;

pub use run::{Progress, start};
pub use table::{TaskId, TaskKind, Tasks};
