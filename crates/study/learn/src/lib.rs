//! The learning line: new senses wait in the learning zone until each has
//! gone through the whole question order.

pub mod rules;
pub mod store;

mod answering;
mod asking;
mod error;
mod filling;
mod item;
mod service;
mod settings;

#[cfg(test)]
mod test_support;

pub use error::LearnError;
pub use item::Item;
pub use service::LearnService;
pub use settings::{LearnOrder, LearnSettings, LearnSettingsError};
