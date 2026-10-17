use serde::{Deserialize, Serialize};

/// The order in which new senses enter the learning zone.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum LearningOrder {
    /// Creation (rowid) order.
    #[default]
    Creation,
    Random,
}
