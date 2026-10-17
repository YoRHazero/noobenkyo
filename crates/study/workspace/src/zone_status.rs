use serde::Serialize;

use crate::{ReviewMode, Zone};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct ZoneStatus {
    pub zone: Zone,
    pub items: u32,
    pub capacity: u32,
    pub new_today: u32,
    pub daily_new_limit: u32,
    pub review_mode: ReviewMode,
    /// Fixed-group mode with every sense of the group gone.
    pub group_finished: bool,
}
