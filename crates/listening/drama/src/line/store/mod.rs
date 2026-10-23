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

/// Rows the chunked statements affected, as the `u32` counts that leave
/// drama. Saturates instead of wrapping; real counts are far below the limit.
fn affected(rows: u64) -> u32 {
    u32::try_from(rows).unwrap_or(u32::MAX)
}
