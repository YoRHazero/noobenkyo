//! The agent group's public face for core: `AgentService`, the app's side of
//! the inbox, and the types its methods use; and the pure functions that
//! write the starting prompts the frontend copies for an agent. The command
//! line for agents uses the basic tools in agent-tools directly, not this.

mod entries;
mod error;
mod list;
mod scripts;
mod service;
mod watch;

pub use agent_tools::{
    FillSentencesTask, ScriptTarget, ScriptTask, WordListTask, fill_sentences_prompt,
    script_prompt, word_list_prompt,
};
pub use error::AgentError;
pub use list::InboxItem;
pub use scripts::InboxScript;
pub use service::AgentService;
pub use watch::InboxWatcher;
