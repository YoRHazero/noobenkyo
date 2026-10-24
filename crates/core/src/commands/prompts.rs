//! The starting prompts the frontend copies for an agent, one command per
//! function of ai, handed on as is. Pure and short, so synchronous.

use ai::{FillSentencesTask, ScriptTask, WordListTask};

/// The prompt for writing a new episode or continuing one.
#[tauri::command]
#[specta::specta]
pub fn ai_script_prompt(task: ScriptTask) -> String {
    ai::script_prompt(&task)
}

/// The prompt for filling in sentences for the rows the filter shows.
#[tauri::command]
#[specta::specta]
pub fn ai_fill_sentences_prompt(task: FillSentencesTask) -> String {
    ai::fill_sentences_prompt(&task)
}

/// The prompt for writing a new word list.
#[tauri::command]
#[specta::specta]
pub fn ai_word_list_prompt(task: WordListTask) -> String {
    ai::word_list_prompt(&task)
}
