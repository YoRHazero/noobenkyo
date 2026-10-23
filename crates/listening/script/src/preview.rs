//! What the script preview shows: the script as it would be stored, each
//! Japanese line also split for ruby, or the problems that stop it.

use listening_drama::text::ruby::parse;
use listening_drama::{RubySegment, ScriptLine};
use serde::Serialize;

use crate::parse_script;

/// A script that passes, ready to show; or why it does not pass.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ScriptPreview {
    Passed {
        title: String,
        summary: String,
        lines: Vec<PreviewLine>,
    },
    /// One line of Chinese each, the same as `check_script` gives.
    Failed { problems: Vec<String> },
}

/// One line of the preview. Its number is its place in `lines`, from 1.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub struct PreviewLine {
    pub speaker: String,
    /// As written, with reading marks: what editing starts from.
    pub ja: String,
    /// `ja` split into text and ruby, for showing it.
    pub ja_ruby: Vec<RubySegment>,
    pub zh: String,
}

/// Parses the script as storing it would, for the preview.
pub fn preview_script(text: &str) -> ScriptPreview {
    match parse_script(text) {
        Ok(script) => ScriptPreview::Passed {
            title: script.title,
            summary: script.summary,
            lines: script.lines.into_iter().map(preview_line).collect(),
        },
        Err(problems) => ScriptPreview::Failed { problems },
    }
}

fn preview_line(line: ScriptLine) -> PreviewLine {
    let ja_ruby = parse(&line.ja).expect("parse_script checks every reading mark");
    PreviewLine {
        speaker: line.speaker,
        ja: line.ja,
        ja_ruby,
        zh: line.zh,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check_script;

    #[test]
    fn a_passing_script_is_shown_as_it_would_be_stored() {
        let text = r#"{"title": " 早晨 ", "summary": "梗概。", "lines": [
            {"speaker": "旁白", "ja": " {春|はる}の朝。", "zh": "春天的早晨。", "note": "忽略"}
        ]}"#;
        assert_eq!(
            preview_script(text),
            ScriptPreview::Passed {
                title: "早晨".into(),
                summary: "梗概。".into(),
                lines: vec![PreviewLine {
                    speaker: "旁白".into(),
                    ja: "{春|はる}の朝。".into(),
                    ja_ruby: vec![
                        RubySegment::Ruby {
                            writing: "春".into(),
                            reading: "はる".into(),
                        },
                        RubySegment::Text {
                            text: "の朝。".into()
                        },
                    ],
                    zh: "春天的早晨。".into(),
                }],
            }
        );
    }

    #[test]
    fn a_failing_script_shows_the_problems_the_check_finds() {
        let text =
            r#"{"title": "早晨", "lines": [{"speaker": "旁白", "ja": "{春|haru}", "zh": ""}]}"#;
        assert_eq!(
            preview_script(text),
            ScriptPreview::Failed {
                problems: check_script(text)
            }
        );
        assert_eq!(check_script(text).len(), 3);
    }
}
