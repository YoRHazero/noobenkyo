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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SentenceContent;
    use crate::store::query::sense_detail::sense_detail;
    use crate::store::tables::label::add_labels;
    use crate::store::tables::sentence::insert_sentence;
    use crate::store::test_db::{memory_db, sense_of};

    fn key(gloss: &str) -> SenseKey {
        SenseKey {
            writing: "諦める".into(),
            origin: String::new(),
            kana: "あきらめる".into(),
            primary_gloss: gloss.into(),
        }
    }

    async fn fixture(db: &DatabaseConnection) {
        let id = sense_of(db, "諦める", "あきらめる", "放弃").await;
        add_labels(db, id, &["N3".to_owned()]).await.unwrap();
        insert_sentence(db, id, "[[{諦|あきら}めた]]", None)
            .await
            .unwrap();
        sense_of(db, "諦める", "あきらめる", "死心").await;
    }

    #[tokio::test]
    async fn replaces_given_fields_and_keeps_the_rest() {
        let db = memory_db().await;
        fixture(&db).await;
        let patch = SensePatch {
            glosses: Some(vec!["放弃（做某事）".into(), "放弃".into()]),
            sentences: Some(vec![SentenceContent {
                marked: "{夢|ゆめ}を[[{諦|あきら}めた]]".into(),
                translation: Some("放弃了梦想".into()),
            }]),
            ..SensePatch::default()
        };
        let replaced = replace_sense(&db, &key("放弃"), &patch).await.unwrap();
        assert_eq!(
            replaced,
            Replaced::Done {
                key: key("放弃（做某事）")
            }
        );
        let detail = sense_detail(&db, &key("放弃（做某事）"))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(detail.glosses, ["放弃（做某事）", "放弃"]);
        assert_eq!(detail.labels, ["N3"]);
        assert_eq!(detail.sentences.len(), 1);
        assert_eq!(
            detail.sentences[0].translation.as_deref(),
            Some("放弃了梦想")
        );
    }

    #[tokio::test]
    async fn rejects_without_changing_anything() {
        let db = memory_db().await;
        fixture(&db).await;
        let patch = SensePatch {
            glosses: Some(vec!["死心".into()]),
            labels: Some(vec![]),
            ..SensePatch::default()
        };
        let replaced = replace_sense(&db, &key("放弃"), &patch).await.unwrap();
        assert!(matches!(replaced, Replaced::Rejected { .. }));
        let detail = sense_detail(&db, &key("放弃")).await.unwrap().unwrap();
        assert_eq!(detail.labels, ["N3"]);
    }

    #[tokio::test]
    async fn reports_a_missing_sense() {
        let db = memory_db().await;
        let replaced = replace_sense(&db, &key("放弃"), &SensePatch::default())
            .await
            .unwrap();
        assert_eq!(replaced, Replaced::NotFound);
    }
}
