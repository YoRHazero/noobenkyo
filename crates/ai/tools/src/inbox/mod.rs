//! The inbox: a folder in the app's data directory. The command line leaves
//! what an agent delivers there, each under the name of the file it came
//! from. The app opens a preview for each. Entries leave the inbox when the
//! app takes them; a script stays until it is saved or discarded, and edits
//! in its preview are written back to it.

mod delivery;
mod folder;

pub use delivery::{Delivery, ScriptTarget};
pub use folder::{Inbox, InboxError};
