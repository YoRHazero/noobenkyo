//! Episodes of a world: a number to sort by, a title, a summary, and lines.

pub mod rules;
pub mod store;

mod values;

pub use values::{EpisodeDetail, EpisodeSummary};
