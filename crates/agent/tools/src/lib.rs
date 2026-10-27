//! The basic tools of the agent group: plain functions that read data, explain
//! formats to a model, start an agent on a task, check what a model wrote,
//! and hand it to the app.
//! Nothing here talks to a model. The command line for agents and the agent
//! service call them.
//!
//! Tools that the feature groups already offer as they are (exporting,
//! listing worlds and episodes, the script context, the script check) are
//! called on their services directly; only what is new lives here.

mod entries;
mod guide;
mod inbox;
mod prompt;
mod send;

pub use entries::{ExistingWord, check_entries, existing_entries};
pub use guide::{dictionary_guide, listening_guide};
pub use inbox::{Delivery, Inbox, InboxError, ScriptTarget};
pub use prompt::{
    FillSentencesTask, ScriptTask, WordListTask, fill_sentences_prompt, script_prompt,
    word_list_prompt,
};
pub use send::{SendError, replace_script, send_entries, send_script};
