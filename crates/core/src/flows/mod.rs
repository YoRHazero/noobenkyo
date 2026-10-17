//! Cross-feature flows. Each step calls a feature and passes values on;
//! decisions come from what the features return.

pub mod admit;
pub mod fill_zone;
pub mod question;
pub mod record_answer;
pub mod render;
pub mod settle;
pub mod start_group;

#[cfg(test)]
mod tests;
