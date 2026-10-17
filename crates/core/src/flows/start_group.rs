use dictionary::DictionaryService;
use study::{StudyService, Zone};

use super::admit::admit;
use crate::command_error::CommandError;
use crate::convert::candidate;

/// Starts a new fixed review group, if study allows one now.
pub async fn start_group(
    dictionary: &DictionaryService,
    study: &mut StudyService,
) -> Result<(), CommandError> {
    let candidates: Vec<_> = dictionary
        .candidates()
        .await?
        .into_iter()
        .map(candidate::to_study)
        .collect();
    let keys = study.group_keys(&candidates);
    admit(dictionary, study, Zone::Review, keys).await
}
