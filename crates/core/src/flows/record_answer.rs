use dictionary::DictionaryService;
use study::AnswerRecord;

use crate::command_error::CommandError;
use crate::convert::log_record;

/// Hands an answer that meets the logging condition to the dictionary, which
/// decides by its switch whether to store it.
pub async fn record_answer(
    dictionary: &DictionaryService,
    record: AnswerRecord,
) -> Result<(), CommandError> {
    dictionary
        .record_answer(&log_record::to_dictionary(record))
        .await?;
    Ok(())
}
