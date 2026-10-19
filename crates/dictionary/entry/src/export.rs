//! Content → entry text, one line per piece of content.

use dictionary_word::Content;

use crate::{Entry, EntrySentence};

/// An exported entry text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exported {
    pub text: String,
    pub entries: u32,
}

/// Writes each piece of content as one line. Empty origin, stratum and
/// translation are left out; progress is written only when kept.
pub fn to_text(contents: &[Content], keep_progress: bool) -> Exported {
    let mut text = String::new();
    for content in contents {
        let line = serde_json::to_string(&to_entry(content, keep_progress))
            .expect("an entry always serializes");
        text.push_str(&line);
        text.push('\n');
    }
    Exported {
        text,
        entries: u32::try_from(contents.len()).unwrap_or(u32::MAX),
    }
}

fn to_entry(content: &Content, keep_progress: bool) -> Entry {
    let mut entry = Entry {
        writing: content.writing.clone(),
        origin: non_empty(&content.origin),
        stratum: non_empty(&content.stratum),
        ..Entry::default()
    };
    let Some(reading) = &content.reading else {
        return entry;
    };
    entry.kana = Some(reading.kana.clone());
    let Some(sense) = &reading.sense else {
        return entry;
    };
    entry.glosses = Some(sense.glosses.clone());
    entry.labels = (!sense.labels.is_empty()).then(|| sense.labels.clone());
    entry.sentences = (!sense.sentences.is_empty()).then(|| {
        sense
            .sentences
            .iter()
            .map(|s| EntrySentence {
                marked: s.marked.clone(),
                translation: s.translation.clone(),
                ..EntrySentence::default()
            })
            .collect()
    });
    if let Some(progress) = sense.progress.as_ref().filter(|_| keep_progress) {
        entry.stage = Some(progress.stage);
        entry.passes = Some(progress.passes);
        entry.lapses = Some(progress.lapses);
        entry.modified_at = progress.modified_at;
    }
    entry
}

fn non_empty(text: &str) -> Option<String> {
    (!text.is_empty()).then(|| text.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::convert;

    const TEXT: &str = concat!(
        r#"{"writing":"諦める","stratum":"和语","kana":"あきらめる","glosses":["放弃","死心"],"labels":["N3"],"sentences":[{"marked":"[[{諦|あきら}めない]]で","translation":"别放弃"}],"stage":2,"passes":3,"lapses":1,"modified_at":"2026-10-17T21:05:02.117+08:00"}"#,
        "\n",
        r#"{"writing":"パソコン","origin":"personal computer"}"#,
        "\n",
        r#"{"writing":"上","kana":"うえ"}"#,
        "\n",
    );

    fn contents() -> Vec<Content> {
        convert(TEXT)
            .entries
            .into_iter()
            .map(|e| e.content)
            .collect()
    }

    #[test]
    fn round_trips_through_convert() {
        let exported = to_text(&contents(), true);
        assert_eq!(exported.text, TEXT);
        assert_eq!(exported.entries, 3);
    }

    #[test]
    fn leaves_out_progress_when_not_kept() {
        let exported = to_text(&contents(), false);
        let first = exported.text.lines().next().unwrap();
        assert!(!first.contains("stage") && !first.contains("modified_at"));
    }
}
