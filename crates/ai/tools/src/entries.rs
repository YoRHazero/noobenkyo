//! Entry tools: the check a model's entries must pass, and which of their
//! words the dictionary already has. Both read the text the way importing a
//! generated list would.

use std::collections::HashSet;

use dictionary::{
    DictionaryError, DictionaryService, WordKey, generated_import_options, preview_entries,
};

/// The dictionary's check with the options for generated lists: warnings on
/// the lines that would be imported, one line of Chinese each. Lines dropped
/// whole are not listed: they are left out of the import rather than fixed,
/// so a model spends no time on them.
pub fn check_entries(text: &str) -> Vec<String> {
    dictionary::check_entries(text, generated_import_options())
}

/// An entry whose word the dictionary already has.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExistingWord {
    /// 1-based line number in the text.
    pub line: u32,
    pub word: WordKey,
}

/// The entries of the text whose word is already in the dictionary, in text
/// order. Lines that would not be imported are left out.
pub async fn existing_entries(
    dictionary: &DictionaryService,
    text: &str,
) -> Result<Vec<ExistingWord>, DictionaryError> {
    let entries: Vec<ExistingWord> = preview_entries(text, generated_import_options())
        .entries
        .into_iter()
        .map(|entry| ExistingWord {
            line: entry.line,
            word: WordKey {
                writing: entry.content.writing,
                origin: entry.content.origin,
            },
        })
        .collect();
    let keys: Vec<WordKey> = entries.iter().map(|entry| entry.word.clone()).collect();
    let stored: HashSet<WordKey> = dictionary
        .existing_words(&keys)
        .await?
        .into_iter()
        .collect();
    Ok(entries
        .into_iter()
        .filter(|entry| stored.contains(&entry.word))
        .collect())
}
