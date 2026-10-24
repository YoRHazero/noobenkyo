//! The starting prompts the frontend copies for an agent, one per task: the
//! task, which guide to run first, and what the user chose on screen. How
//! to go about it is left to the guide and the agent; what the user left
//! open is said to be the agent's to judge.

mod fill_sentences;
mod script;
mod shell_word;
mod word_list;

pub use fill_sentences::{FillSentencesTask, fill_sentences_prompt};
pub use script::{ScriptTask, script_prompt};
pub use word_list::{WordListTask, word_list_prompt};
