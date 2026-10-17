use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum ReviewMode {
    /// Fixed capacity; every exit is refilled at once.
    #[default]
    Endless,
    /// A group chosen at the start; no refills until the user starts a new group.
    FixedGroup,
}
