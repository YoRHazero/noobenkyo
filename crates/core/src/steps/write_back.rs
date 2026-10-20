use dictionary::DictionaryService;
use study::WriteBack;

use crate::command_error::CommandError;
use crate::convert::sense_key;

/// Hands study's progress to the dictionary as is. A sense deleted or
/// renamed meanwhile is not found, and is left alone.
pub async fn write_back(
    dictionary: &DictionaryService,
    write: WriteBack,
) -> Result<(), CommandError> {
    dictionary
        .update_progress(
            &sense_key::to_dictionary(write.key),
            write.stage,
            write.passes,
            write.lapses,
        )
        .await?;
    Ok(())
}
