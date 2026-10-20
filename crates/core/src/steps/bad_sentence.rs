use dictionary::DictionaryService;
use study::BadSentence;

use crate::command_error::CommandError;
use crate::convert::sense_key;

/// Deletes the sentence reported as wrong. One already gone is left alone.
pub async fn delete_bad_sentence(
    dictionary: &DictionaryService,
    bad: BadSentence,
) -> Result<(), CommandError> {
    dictionary
        .delete_sentence(&sense_key::to_dictionary(bad.key), &bad.marked)
        .await?;
    Ok(())
}
