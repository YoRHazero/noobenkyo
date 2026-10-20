use sea_orm::{ConnectionTrait, DbErr};

use crate::SenseKey;
use crate::rules::sentence;
use crate::store::by_key;
use crate::store::tables::sentence as sentence_table;

/// Deletes the sentence of a sense whose plain text equals that of
/// `marked`. `false` if there is no such sentence.
pub async fn delete_sentence(
    db: &impl ConnectionTrait,
    key: &SenseKey,
    marked: &str,
) -> Result<bool, DbErr> {
    let Some(sense) = by_key::find_sense(db, key).await? else {
        return Ok(false);
    };
    let Some(plain) = plain_of(marked) else {
        return Ok(false);
    };
    for row in sentence_table::sentences_of_sense(db, sense.id).await? {
        if plain_of(&row.marked).as_deref() == Some(plain.as_str()) {
            sentence_table::delete_sentence(db, row.id).await?;
            return Ok(true);
        }
    }
    Ok(false)
}

fn plain_of(marked: &str) -> Option<String> {
    sentence::parse(marked)
        .ok()
        .map(|segments| sentence::plain_text(&segments))
}
