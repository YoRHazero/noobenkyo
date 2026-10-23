use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TransactionTrait};

use crate::by_key;
use crate::character::rules::check::check_character;
use crate::entity::{character, reference};
use crate::{DramaError, NewCharacter};

/// Creates a character with its references. Returns the problems that
/// rejected it, empty when it was created.
pub async fn create_character(
    db: &DatabaseConnection,
    new: &NewCharacter,
) -> Result<Vec<String>, DramaError> {
    let checked = match check_character(new) {
        Ok(checked) => checked,
        Err(problems) => return Ok(problems),
    };
    let txn = db.begin().await?;
    if by_key::find_character(&txn, &checked.name).await?.is_some() {
        return Ok(vec![format!("角色 {} 已经存在", checked.name)]);
    }
    let created = character::ActiveModel {
        name: Set(checked.name),
        kana: Set(checked.kana),
        prompt: Set(checked.prompt),
        gender: Set(checked.gender.code().to_owned()),
        age: Set(i64::from(checked.age)),
        ..Default::default()
    }
    .insert(&txn)
    .await?;
    for each in checked.references {
        reference::ActiveModel {
            character_id: Set(created.id),
            language: Set(each.language.code().to_owned()),
            audio: Set(each.audio.to_vec()),
            text: Set(each.text),
        }
        .insert(&txn)
        .await?;
    }
    txn.commit().await?;
    Ok(Vec::new())
}
