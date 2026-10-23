use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, TransactionTrait};

use crate::by_key;
use crate::character::store::languages::languages_by_name;
use crate::entity::{line, line_audio};
use crate::line::rules::text::check_text;
use crate::line::rules::voice::voice_problem;
use crate::{DramaError, Language, LineId};

/// Replaces one language of a line and drops its audio in that language.
/// Its voice must still be able to synthesize the language. Returns the
/// problems that rejected the text, empty when it was replaced or unchanged.
pub async fn update_line(
    db: &DatabaseConnection,
    id: LineId,
    language: Language,
    text: &str,
) -> Result<Vec<String>, DramaError> {
    let text = match check_text(language, text) {
        Ok(text) => text,
        Err(problems) => return Ok(problems),
    };
    let txn = db.begin().await?;
    let found = by_key::require_line(&txn, id).await?;
    let current = match language {
        Language::Ja => &found.ja,
        Language::Zh => &found.zh,
    };
    if *current == text {
        return Ok(Vec::new());
    }
    let languages = languages_by_name(&txn, std::slice::from_ref(&found.voice)).await?;
    let languages = languages.get(&found.voice).map(Vec::as_slice);
    if let Some(problem) = voice_problem(&found.voice, language, languages) {
        return Ok(vec![problem]);
    }
    let mut changed = line::ActiveModel {
        id: Set(found.id),
        ..Default::default()
    };
    match language {
        Language::Ja => changed.ja = Set(text),
        Language::Zh => changed.zh = Set(text),
    }
    changed.update(&txn).await?;
    line_audio::Entity::delete_by_id((found.id, language.code().to_owned()))
        .exec(&txn)
        .await?;
    txn.commit().await?;
    Ok(Vec::new())
}
