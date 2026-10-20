use dictionary::{Completeness, ContentFilter, DictionaryService, ProgressState, SenseOrder};
use study::StudyService;

use crate::command_error::CommandError;
use crate::convert::sense_key;

/// Tops the learning zone up when it has room. Every untouched sense with
/// sentences is offered; the learning line picks which ones enter.
pub async fn fill_zone(
    dictionary: &DictionaryService,
    study: &mut StudyService,
) -> Result<(), CommandError> {
    if study.learn().is_full() {
        return Ok(());
    }
    let filter = ContentFilter {
        text: None,
        completeness: vec![Completeness::WithSentence],
        progress: ProgressState::Untouched,
    };
    let keys = dictionary
        .filter_senses(&filter, &[], SenseOrder::Creation, None)
        .await?
        .into_iter()
        .map(|sense| sense_key::to_study(sense.key))
        .collect();
    Ok(study.learn().fill(keys)?)
}
