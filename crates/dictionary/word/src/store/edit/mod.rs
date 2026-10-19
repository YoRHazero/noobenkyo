//! Changing a sense in place.

pub mod replace_sense;
pub mod update_progress;

mod replaced;
mod sense_patch;

pub use replaced::Replaced;
pub use sense_patch::SensePatch;
