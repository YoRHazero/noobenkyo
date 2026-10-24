//! The ai group's public face for core: `AiService`, the app's side of the
//! inbox, and the types its methods use. The command line for agents uses
//! the basic tools in ai-tools directly, not this. Calling the dedicated
//! backend will be added here later.

mod entries;
mod error;
mod list;
mod scripts;
mod service;
mod watch;

pub use ai_tools::ScriptTarget;
pub use error::AiError;
pub use list::InboxItem;
pub use scripts::InboxScript;
pub use service::AiService;
pub use watch::InboxWatcher;
