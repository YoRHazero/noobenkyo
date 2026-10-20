use dictionary::DictionaryService;

use crate::command_error::CommandError;
use crate::convert::{content, sense_key};

/// The content study asked for; `None` when the sense is gone.
pub async fn content_of(
    dictionary: &DictionaryService,
    key: &study::SenseKey,
) -> Result<Option<study::Content>, CommandError> {
    let detail = dictionary
        .sense_detail(&sense_key::to_dictionary(key.clone()))
        .await?;
    Ok(detail.map(content::to_study))
}
