use sea_orm::{DatabaseConnection, DbErr, TransactionTrait};

use super::{Replaced, SensePatch};
use crate::SenseKey;
use crate::rules::patch::check_patch;
use crate::store::by_key;
use crate::store::tables::{label, sense, sentence};

/// Replaces the given fields of a sense in one transaction. Any problem
/// leaves the sense untouched.
pub async fn replace_sense(
    db: &DatabaseConnection,
    key: &SenseKey,
    patch: &SensePatch,
) -> Result<Replaced, DbErr> {
    let txn = db.begin().await?;
    let Some(target) = by_key::find_sense(&txn, key).await? else {
        return Ok(Replaced::NotFound);
    };
    let others: Vec<Vec<String>> = sense::senses_of_reading(&txn, target.reading_id)
        .await?
        .iter()
        .filter(|other| other.id != target.id)
        .map(sense::glosses)
        .collect();
    let checked = match check_patch(patch, &others) {
        Ok(checked) => checked,
        Err(problems) => return Ok(Replaced::Rejected { problems }),
    };
    if let Some(glosses) = &checked.glosses {
        sense::set_glosses(&txn, target.id, &glosses[0], &glosses[1..]).await?;
    }
    if let Some(labels) = &checked.labels {
        label::replace_labels(&txn, target.id, labels).await?;
    }
    if let Some(sentences) = &checked.sentences {
        sentence::replace_sentences(&txn, target.id, sentences).await?;
    }
    txn.commit().await?;
    let primary_gloss = match checked.glosses {
        Some(glosses) => glosses[0].clone(),
        None => target.primary_gloss,
    };
    Ok(Replaced::Done {
        key: SenseKey {
            primary_gloss,
            ..key.clone()
        },
    })
}
