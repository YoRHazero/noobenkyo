//! Merging content into the dictionary, layer by layer.

mod contents;
mod layers;
mod outcome;

pub use contents::merge_contents;
pub use outcome::{MergeOutcome, SenseOutcome};
