//! The basic tools of the agent group: plain functions that read data,
//! explain formats to a model, check what a model wrote, and hand it to the
//! app through the inbox. Nothing here talks to a model. The command line
//! for agents calls them.
//!
//! Tools that the feature groups already offer as they are (exporting,
//! listing worlds and episodes, the script context, the script check,
//! reading the inbox) are called on their services directly; only what is
//! new lives here.

mod entries;
mod guide;
mod send;

pub use entries::{ExistingWord, check_entries, existing_entries};
pub use guide::{dictionary_guide, listening_guide};
pub use send::{SendError, replace_script, send_entries, send_script};
