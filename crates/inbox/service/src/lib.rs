//! The inbox group's public face: `InboxService` and the types its methods
//! use. What agents and, later, the app's own models deliver waits here
//! until the user has looked it over. Each source has its own folder in the
//! app's data directory, and each delivery is one file there. Only the file
//! format and the rules for keeping the files live here: checking what is
//! delivered and storing it in a feature are the callers' business.

mod delivery;
mod entries;
mod error;
mod folder;
mod list;
mod put;
mod scripts;
mod service;
mod source;
mod watch;

pub use delivery::{Delivery, ScriptTarget};
pub use error::InboxError;
pub use list::InboxItem;
pub use scripts::InboxScript;
pub use service::InboxService;
pub use source::InboxSource;
pub use watch::InboxWatcher;
