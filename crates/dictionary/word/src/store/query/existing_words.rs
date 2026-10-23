//! Which of some words are already in the dictionary.

use std::collections::HashSet;

use sea_orm::{ConnectionTrait, DbErr};

use crate::WordKey;
use crate::store::tables::word as word_table;

/// The given words that are in the dictionary, matched exactly by writing
/// and origin, in the order given and each once.
pub async fn existing_words(
    db: &impl ConnectionTrait,
    keys: &[WordKey],
) -> Result<Vec<WordKey>, DbErr> {
    let writings: Vec<&str> = keys.iter().map(|key| key.writing.as_str()).collect();
    let stored: HashSet<WordKey> = word_table::words_with_writings(db, &writings)
        .await?
        .into_iter()
        .map(|word| WordKey {
            writing: word.writing,
            origin: word.origin,
        })
        .collect();
    let mut seen = HashSet::new();
    Ok(keys
        .iter()
        .filter(|key| stored.contains(*key) && seen.insert(*key))
        .cloned()
        .collect())
}
