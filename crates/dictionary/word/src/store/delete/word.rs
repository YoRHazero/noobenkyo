use sea_orm::{ConnectionTrait, DbErr};

use crate::WordKey;
use crate::store::by_key;
use crate::store::tables::word;

/// Deletes a word with its readings, senses, labels and sentences.
/// `false` if the word is already gone.
pub async fn delete_word(db: &impl ConnectionTrait, key: &WordKey) -> Result<bool, DbErr> {
    let Some(found) = by_key::find_word(db, key).await? else {
        return Ok(false);
    };
    word::delete_word(db, found.id).await?;
    Ok(true)
}
