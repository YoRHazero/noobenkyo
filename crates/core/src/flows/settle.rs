use dictionary::DictionaryService;
use study::{Settlement, StudyService, Zone};

use crate::command_error::CommandError;
use crate::convert::sense_key;

/// Writes a leaving sense's progress to the dictionary, then removes it from the zone.
pub async fn settle_and_remove(
    dictionary: &DictionaryService,
    study: &mut StudyService,
    zone: Zone,
    settlement: Settlement,
) -> Result<(), CommandError> {
    write_back(dictionary, &settlement).await?;
    study.remove(zone, &settlement.key)?;
    Ok(())
}

/// Writes a settlement to the dictionary. A sense deleted meanwhile is skipped by the dictionary.
pub async fn write_back(
    dictionary: &DictionaryService,
    settlement: &Settlement,
) -> Result<(), CommandError> {
    let key = sense_key::to_dictionary(settlement.key.clone());
    let progress = settlement.progress;
    dictionary
        .settle(&key, progress.stage, progress.passes, progress.lapses)
        .await?;
    Ok(())
}
