//! Reading marks in Japanese lines: `{writing|reading}`, where the writing is
//! not empty and the reading is kana. Outside marks there is no lone `{`,
//! `|` or `}`.

use serde::Serialize;
use thiserror::Error;

use super::kana::is_kana;

/// A piece of a Japanese line: plain text, or a writing with its reading,
/// shown as ruby.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum RubySegment {
    Text { text: String },
    Ruby { writing: String, reading: String },
}

/// Why a Japanese line's reading marks are malformed. Shown to the user and
/// to the model.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum RubyError {
    #[error("读音标记 {0} 没有闭合")]
    Unclosed(String),
    #[error("读音标记 {0} 缺少 |")]
    NoBar(String),
    #[error("读音标记 {0} 有多个 |")]
    SeveralBars(String),
    #[error("读音标记 {0} 的写法为空")]
    EmptyWriting(String),
    #[error("读音标记 {0} 的读音为空")]
    EmptyReading(String),
    #[error("读音标记 {mark} 的读音只能有假名，不能有 {found}")]
    NotKana { mark: String, found: char },
    #[error("标记以外有单独的 {0}")]
    Stray(char),
}

/// Splits a Japanese line into text and reading marks.
pub fn parse(ja: &str) -> Result<Vec<RubySegment>, RubyError> {
    let chars: Vec<char> = ja.chars().collect();
    let mut segments = Vec::new();
    let mut text = String::new();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '{' => {
                if !text.is_empty() {
                    segments.push(RubySegment::Text {
                        text: std::mem::take(&mut text),
                    });
                }
                let (segment, consumed) = parse_mark(&chars[i..])?;
                segments.push(segment);
                i += consumed;
            }
            c @ ('|' | '}') => return Err(RubyError::Stray(c)),
            c => {
                text.push(c);
                i += 1;
            }
        }
    }
    if !text.is_empty() {
        segments.push(RubySegment::Text { text });
    }
    Ok(segments)
}

/// Parses the mark that starts `chars`; returns it with the number of chars consumed.
fn parse_mark(chars: &[char]) -> Result<(RubySegment, usize), RubyError> {
    let end = chars[1..].iter().position(|c| matches!(c, '{' | '}'));
    let Some(close) = end.map(|offset| offset + 1).filter(|at| chars[*at] == '}') else {
        let until = end.map_or(chars.len(), |offset| offset + 1);
        return Err(RubyError::Unclosed(chars[..until].iter().collect()));
    };
    let mark: String = chars[..=close].iter().collect();
    let inner = &chars[1..close];
    let bars: Vec<usize> = (0..inner.len()).filter(|at| inner[*at] == '|').collect();
    let bar = match bars.as_slice() {
        [] => return Err(RubyError::NoBar(mark)),
        [bar] => *bar,
        _ => return Err(RubyError::SeveralBars(mark)),
    };
    let writing: String = inner[..bar].iter().collect();
    let reading: String = inner[bar + 1..].iter().collect();
    if writing.is_empty() {
        return Err(RubyError::EmptyWriting(mark));
    }
    if reading.is_empty() {
        return Err(RubyError::EmptyReading(mark));
    }
    if let Some(found) = reading.chars().find(|c| !is_kana(*c)) {
        return Err(RubyError::NotKana { mark, found });
    }
    Ok((RubySegment::Ruby { writing, reading }, close + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ruby(writing: &str, reading: &str) -> RubySegment {
        RubySegment::Ruby {
            writing: writing.into(),
            reading: reading.into(),
        }
    }

    #[test]
    fn parses_marks_and_text() {
        assert_eq!(
            parse("{明日|あした}は{美咲|みさき}と会う。").unwrap(),
            vec![
                ruby("明日", "あした"),
                RubySegment::Text { text: "は".into() },
                ruby("美咲", "みさき"),
                RubySegment::Text {
                    text: "と会う。".into()
                },
            ]
        );
        assert_eq!(
            parse("会う。").unwrap(),
            vec![RubySegment::Text {
                text: "会う。".into()
            }]
        );
        assert_eq!(parse("").unwrap(), vec![]);
    }

    #[test]
    fn readings_may_be_katakana() {
        assert_eq!(
            parse("{J・K|ジェイ・ケイ}").unwrap(),
            vec![ruby("J・K", "ジェイ・ケイ")]
        );
    }

    #[test]
    fn rejects_malformed_marks() {
        let cases = [
            ("{明日|あした", RubyError::Unclosed("{明日|あした".into())),
            ("{明{日|あした}", RubyError::Unclosed("{明".into())),
            ("{明日}", RubyError::NoBar("{明日}".into())),
            (
                "{明日|あ|した}",
                RubyError::SeveralBars("{明日|あ|した}".into()),
            ),
            ("{|あした}", RubyError::EmptyWriting("{|あした}".into())),
            ("{明日|}", RubyError::EmptyReading("{明日|}".into())),
            (
                "{明日|ashita}",
                RubyError::NotKana {
                    mark: "{明日|ashita}".into(),
                    found: 'a',
                },
            ),
            (
                "{明日|あ した}",
                RubyError::NotKana {
                    mark: "{明日|あ した}".into(),
                    found: ' ',
                },
            ),
            ("明日}", RubyError::Stray('}')),
            ("明日|あした", RubyError::Stray('|')),
        ];
        for (ja, error) in cases {
            assert_eq!(parse(ja).unwrap_err(), error, "{ja}");
        }
    }
}
