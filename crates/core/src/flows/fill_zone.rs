use dictionary::DictionaryService;
use study::{StudyService, Zone};

use super::admit::admit;
use crate::command_error::CommandError;
use crate::convert::candidate;

/// Tops the zone up if study wants more senses.
pub async fn fill_zone(
    dictionary: &DictionaryService,
    study: &mut StudyService,
    zone: Zone,
) -> Result<(), CommandError> {
    if !study.wants_more(zone) {
        return Ok(());
    }
    let candidates: Vec<_> = dictionary
        .candidates()
        .await?
        .into_iter()
        .map(candidate::to_study)
        .collect();
    let keys = study.wanted(zone, &candidates);
    admit(dictionary, study, zone, keys).await
}
