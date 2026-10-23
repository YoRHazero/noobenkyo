//! The inbox: a folder in the app's data directory. The command line leaves
//! what an agent delivers there; the app takes the deliveries one at a time,
//! oldest first, and opens a preview for each.

mod delivery;
mod folder;

pub use delivery::{Delivery, ScriptTarget};
pub use folder::{Inbox, InboxError};
