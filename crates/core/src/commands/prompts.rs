//! The starting prompts the frontend copies for an agent, one command per
//! function of agent, handed on as is. Pure and short, so synchronous.

use agent::{FillSentencesTask, ScriptTask, WordListTask};

/// The prompt for writing a new episode or continuing one.
#[tauri::command]
#[specta::specta]
pub fn agent_script_prompt(task: ScriptTask) -> String {
    agent::script_prompt(&task)
}

/// The prompt for filling in sentences for the rows the filter shows.
#[tauri::command]
#[specta::specta]
pub fn agent_fill_sentences_prompt(task: FillSentencesTask) -> String {
    agent::fill_sentences_prompt(&task)
}

/// The prompt for writing a new word list.
#[tauri::command]
#[specta::specta]
pub fn agent_word_list_prompt(task: WordListTask) -> String {
    agent::word_list_prompt(&task)
}
