//! The rules a sense patch must meet before anything is written.

use super::merge::sense_match::{SenseMatch, match_sense};
use super::sentence;
use super::text_list::clean_list;
use crate::{SensePatch, SentenceContent};

/// A patch whose values passed every rule, cleaned up, ready to write.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckedPatch {
    /// Non-empty; the first is the primary gloss.
    pub glosses: Option<Vec<String>>,
    pub labels: Option<Vec<String>>,
    pub sentences: Option<Vec<SentenceContent>>,
}

/// Cleans the patch the way import cleans an entry, and reports every
/// problem. `others` are the gloss lists of the other senses of the reading.
pub fn check_patch(
    patch: &SensePatch,
    others: &[Vec<String>],
) -> Result<CheckedPatch, Vec<String>> {
    let mut problems = Vec::new();
    let glosses = patch.glosses.clone().map(clean_list);
    if let Some(glosses) = &glosses {
        if glosses.is_empty() {
            problems.push("释义不能为空".to_owned());
        } else if let SenseMatch::Existing(_) | SenseMatch::Conflict(_) =
            match_sense(others, glosses)
        {
            let primaries: Vec<&str> = others
                .iter()
                .filter(|other| other.iter().any(|gloss| glosses.contains(gloss)))
                .map(|other| other[0].as_str())
                .collect();
            problems.push(format!(
                "释义和同一读音下的其他义项有交集：{}",
                primaries.join("、")
            ));
        }
    }
    let sentences = patch
        .sentences
        .as_ref()
        .map(|sentences| checked_sentences(sentences, &mut problems));
    if !problems.is_empty() {
        return Err(problems);
    }
    Ok(CheckedPatch {
        glosses,
        labels: patch.labels.clone().map(clean_list),
        sentences,
    })
}

/// Valid sentences without repeated plain texts; each invalid one becomes a problem.
fn checked_sentences(
    sentences: &[SentenceContent],
    problems: &mut Vec<String>,
) -> Vec<SentenceContent> {
    let mut kept: Vec<(String, SentenceContent)> = Vec::new();
    for raw in sentences {
        let marked = raw.marked.trim().to_owned();
        let segments = match sentence::parse_checked(&marked) {
            Ok(segments) => segments,
            Err(error) => {
                problems.push(format!("例句 {marked}：{error}"));
                continue;
            }
        };
        let plain = sentence::plain_text(&segments);
        if kept.iter().any(|(kept_plain, _)| *kept_plain == plain) {
            continue;
        }
        let translation = raw
            .translation
            .as_deref()
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .map(str::to_owned);
        kept.push((
            plain,
            SentenceContent {
                marked,
                translation,
            },
        ));
    }
    kept.into_iter().map(|(_, sentence)| sentence).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sentence(marked: &str) -> SentenceContent {
        SentenceContent {
            marked: marked.into(),
            translation: Some(" 译文 ".into()),
        }
    }

    #[test]
    fn untouched_fields_stay_none() {
        assert_eq!(
            check_patch(&SensePatch::default(), &[]),
            Ok(CheckedPatch {
                glosses: None,
                labels: None,
                sentences: None
            })
        );
    }

    #[test]
    fn cleans_lists_and_sentences() {
        let patch = SensePatch {
            glosses: Some(vec![" 放弃 ".into(), "死心".into(), "放弃".into()]),
            labels: Some(vec!["".into(), "N3".into()]),
            sentences: Some(vec![
                sentence("[[{諦|あきら}めた]]"),
                sentence(" [[{諦|あきら}めた]] "),
            ]),
        };
        let checked = check_patch(&patch, &[vec!["上面".into()]]).unwrap();
        assert_eq!(checked.glosses.unwrap(), ["放弃", "死心"]);
        assert_eq!(checked.labels.unwrap(), ["N3"]);
        assert_eq!(
            checked.sentences.unwrap(),
            [SentenceContent {
                marked: "[[{諦|あきら}めた]]".into(),
                translation: Some("译文".into()),
            }]
        );
    }

    #[test]
    fn reports_every_problem() {
        let patch = SensePatch {
            glosses: Some(vec!["明天".into()]),
            labels: None,
            sentences: Some(vec![sentence("[[明日]]"), sentence("明日")]),
        };
        let problems = check_patch(&patch, &[vec!["明天".into(), "翌日".into()]]).unwrap_err();
        assert_eq!(
            problems,
            [
                "释义和同一读音下的其他义项有交集：明天",
                "例句 [[明日]]：目标词里的汉字 明 没有注音",
                "例句 明日：目标词必须恰好出现一次，实际出现 0 次",
            ]
        );
    }

    #[test]
    fn rejects_empty_glosses() {
        let patch = SensePatch {
            glosses: Some(vec![" ".into()]),
            ..SensePatch::default()
        };
        assert_eq!(check_patch(&patch, &[]).unwrap_err(), ["释义不能为空"]);
    }
}
