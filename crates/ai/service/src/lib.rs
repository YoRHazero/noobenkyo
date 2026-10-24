//! The ai group's public face for core: `AiService`, the app's side of the
//! inbox, and the types its methods use; and the pure functions that write
//! the starting prompts the frontend copies for an agent. The command line
//! for agents uses the basic tools in ai-tools directly, not this. Calling the dedicated
//! backend will be added here later.

mod entries;
mod error;
mod list;
mod scripts;
mod service;
mod watch;

pub use ai_tools::{
    FillSentencesTask, ScriptTarget, ScriptTask, WordListTask, fill_sentences_prompt,
    script_prompt, word_list_prompt,
};
pub use error::AiError;
pub use list::InboxItem;
pub use scripts::InboxScript;
pub use service::AiService;
pub use watch::InboxWatcher;
