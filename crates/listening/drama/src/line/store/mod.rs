//! Reads and writes of the line and line audio tables.

pub mod audio;
pub mod clear_audio;
pub mod delete;
pub mod insert;
pub mod missing_audio;
pub mod of_episode;
pub mod save_audio;
pub mod update;

/// Ids per statement when deleting by id; SQLite limits bound values.
const ID_CHUNK: usize = 500;
