use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{Item, ReviewMode, Zone};

/// Everything persisted in workspace.json. In fixed-group mode the review
/// items are the group.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceState {
    pub items: Vec<Item>,
    pub review_mode: ReviewMode,
    /// The day `new_today` counts for.
    pub day: NaiveDate,
    /// Senses that entered the learning zone on `day`.
    pub new_today: u32,
}

impl WorkspaceState {
    pub fn items_in(&self, zone: Zone) -> impl Iterator<Item = &Item> {
        self.items.iter().filter(move |item| item.zone == zone)
    }

    pub fn count(&self, zone: Zone) -> u32 {
        u32::try_from(self.items_in(zone).count()).unwrap_or(u32::MAX)
    }

    /// New senses that entered the learning zone today.
    pub fn new_on(&self, today: NaiveDate) -> u32 {
        if self.day == today { self.new_today } else { 0 }
    }
}
