use serde::{Deserialize, Serialize};

/// The order senses come out in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum SenseOrder {
    /// Creation order.
    Creation,
    Random,
}
