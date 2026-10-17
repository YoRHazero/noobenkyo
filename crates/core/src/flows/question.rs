use dictionary::DictionaryService;
use study::{StudyService, Zone};

use super::fill_zone::fill_zone;
use super::render::render;
use crate::command_error::CommandError;
use crate::payloads::QuestionPayload;

/// Tops the zone up, then returns its current question with the rendered sentence.
pub async fn current_question(
    dictionary: &DictionaryService,
    study: &mut StudyService,
    zone: Zone,
) -> Result<Option<QuestionPayload>, CommandError> {
    fill_zone(dictionary, study, zone).await?;
    let Some(question) = study.current_question(zone) else {
        return Ok(None);
    };
    Ok(Some(QuestionPayload {
        view: question.view,
        fragments: render(question.sentence)?,
    }))
}
