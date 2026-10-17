use dictionary::DictionaryService;
use study::{SenseKey, StudyService, Zone};

use crate::command_error::CommandError;
use crate::convert::{sense_key, snapshot};

/// Loads snapshots of the chosen senses from the dictionary and hands them to study.
pub async fn admit(
    dictionary: &DictionaryService,
    study: &mut StudyService,
    zone: Zone,
    keys: Vec<SenseKey>,
) -> Result<(), CommandError> {
    if keys.is_empty() {
        return Ok(());
    }
    let keys: Vec<_> = keys.into_iter().map(sense_key::to_dictionary).collect();
    let snapshots = dictionary.snapshots(&keys).await?;
    study.admit(
        zone,
        snapshots.into_iter().map(snapshot::to_study).collect(),
    )?;
    Ok(())
}
