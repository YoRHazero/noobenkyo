use dictionary_word::SentenceContent;
use dictionary_word::rules::sentence::{self, SentenceError};

use crate::EntrySentence;

/// A sentence that is dropped, with the reason.
#[derive(Debug)]
pub struct DroppedSentence {
    pub marked: String,
    pub error: SentenceError,
}

/// Keeps valid sentences, dropping later ones whose plain text repeats, and
/// returns the invalid ones separately. An empty translation counts as none.
pub fn split(raw: Vec<EntrySentence>) -> (Vec<SentenceContent>, Vec<DroppedSentence>) {
    let mut kept: Vec<(String, SentenceContent)> = Vec::new();
    let mut dropped = Vec::new();
    for sentence in raw {
        let marked = sentence.marked.trim().to_owned();
        let segments = match sentence::parse_checked(&marked) {
            Ok(segments) => segments,
            Err(error) => {
                dropped.push(DroppedSentence { marked, error });
                continue;
            }
        };
        let plain = sentence::plain_text(&segments);
        if kept.iter().any(|(kept_plain, _)| *kept_plain == plain) {
            continue;
        }
        let translation = sentence
            .translation
            .map(|t| t.trim().to_owned())
            .filter(|t| !t.is_empty());
        kept.push((
            plain,
            SentenceContent {
                marked,
                translation,
            },
        ));
    }
    (
        kept.into_iter().map(|(_, content)| content).collect(),
        dropped,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raw(marked: &str, translation: Option<&str>) -> EntrySentence {
        EntrySentence {
            marked: marked.into(),
            translation: translation.map(str::to_owned),
            ..EntrySentence::default()
        }
    }

    #[test]
    fn keeps_valid_and_drops_invalid_sentences() {
        let (kept, dropped) = split(vec![
            raw(" 夢を[[{諦|あきら}めた]] ", Some(" 放弃了梦想 ")),
            raw("[[諦めない]]で", None),
            raw("{夢|ゆめ}を[[{諦|あきら}めた]]", Some("重复")),
            raw("[[{諦|あきら}める]]", Some(" ")),
        ]);
        assert_eq!(
            kept,
            [
                SentenceContent {
                    marked: "夢を[[{諦|あきら}めた]]".into(),
                    translation: Some("放弃了梦想".into()),
                },
                SentenceContent {
                    marked: "[[{諦|あきら}める]]".into(),
                    translation: None,
                },
            ]
        );
        assert_eq!(dropped.len(), 1);
        assert_eq!(dropped[0].marked, "[[諦めない]]で");
        assert_eq!(
            dropped[0].error,
            SentenceError::TargetKanjiWithoutRuby('諦')
        );
    }
}
