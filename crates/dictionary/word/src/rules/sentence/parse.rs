//! Syntactic parsing of a marked sentence. Content rules live in `check`.

use super::{Piece, Ruby, Segment, SentenceError};

pub fn parse(marked: &str) -> Result<Vec<Segment>, SentenceError> {
    let chars: Vec<char> = marked.chars().collect();
    let mut segments = Vec::new();
    let mut target: Option<Vec<Piece>> = None;
    let mut text = String::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        if c == '[' && next == Some('[') {
            if target.is_some() {
                return Err(SentenceError::NestedTarget);
            }
            flush_text(&mut text, &mut segments, &mut target);
            target = Some(Vec::new());
            i += 2;
        } else if c == ']' && next == Some(']') {
            let Some(mut pieces) = target.take() else {
                return Err(SentenceError::StrayTargetClose);
            };
            if !text.is_empty() {
                pieces.push(Piece::Text(std::mem::take(&mut text)));
            }
            if pieces.is_empty() {
                return Err(SentenceError::EmptyTarget);
            }
            segments.push(Segment::Target(pieces));
            i += 2;
        } else if c == '{' {
            flush_text(&mut text, &mut segments, &mut target);
            let (ruby, consumed) = parse_ruby(&chars[i..])?;
            match target.as_mut() {
                Some(pieces) => pieces.push(Piece::Ruby(ruby)),
                None => segments.push(Segment::Ruby(ruby)),
            }
            i += consumed;
        } else if c == '}' || c == '|' {
            return Err(SentenceError::StrayMark(c));
        } else {
            text.push(c);
            i += 1;
        }
    }

    if target.is_some() {
        return Err(SentenceError::UnclosedTarget);
    }
    flush_text(&mut text, &mut segments, &mut target);
    Ok(segments)
}

/// Moves pending text into the open target, or to the top level.
fn flush_text(text: &mut String, segments: &mut Vec<Segment>, target: &mut Option<Vec<Piece>>) {
    if text.is_empty() {
        return;
    }
    let taken = std::mem::take(text);
    match target.as_mut() {
        Some(pieces) => pieces.push(Piece::Text(taken)),
        None => segments.push(Segment::Text(taken)),
    }
}

/// Parses `{base|kana}` at the start of `chars`; returns it with the number of chars consumed.
fn parse_ruby(chars: &[char]) -> Result<(Ruby, usize), SentenceError> {
    let close = chars
        .iter()
        .position(|c| *c == '}')
        .ok_or(SentenceError::UnclosedRuby)?;
    let inner = &chars[1..close];
    if inner.iter().any(|c| matches!(c, '{' | '[' | ']')) {
        return Err(SentenceError::MalformedRuby);
    }
    let bar = inner
        .iter()
        .position(|c| *c == '|')
        .ok_or(SentenceError::MalformedRuby)?;
    let base: String = inner[..bar].iter().collect();
    let kana: String = inner[bar + 1..].iter().collect();
    if base.is_empty() || kana.is_empty() || kana.contains('|') {
        return Err(SentenceError::MalformedRuby);
    }
    Ok((Ruby { base, kana }, close + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ruby(base: &str, kana: &str) -> Ruby {
        Ruby {
            base: base.into(),
            kana: kana.into(),
        }
    }

    #[test]
    fn parses_ruby_text_and_target() {
        let segments = parse("{何度|なんど}も、[[{諦|あきら}めない]]でね。").unwrap();
        assert_eq!(
            segments,
            vec![
                Segment::Ruby(ruby("何度", "なんど")),
                Segment::Text("も、".into()),
                Segment::Target(vec![
                    Piece::Ruby(ruby("諦", "あきら")),
                    Piece::Text("めない".into()),
                ]),
                Segment::Text("でね。".into()),
            ]
        );
    }

    #[test]
    fn keeps_single_brackets_as_text() {
        assert_eq!(parse("a[b]c").unwrap(), vec![Segment::Text("a[b]c".into())]);
    }

    #[test]
    fn rejects_malformed_marks() {
        assert_eq!(
            parse("{漢字かな}").unwrap_err(),
            SentenceError::MalformedRuby
        );
        assert_eq!(
            parse("{漢字|かな").unwrap_err(),
            SentenceError::UnclosedRuby
        );
        assert_eq!(parse("{|かな}").unwrap_err(), SentenceError::MalformedRuby);
        assert_eq!(parse("あ}").unwrap_err(), SentenceError::StrayMark('}'));
        assert_eq!(parse("[[あ").unwrap_err(), SentenceError::UnclosedTarget);
        assert_eq!(parse("あ]]").unwrap_err(), SentenceError::StrayTargetClose);
        assert_eq!(
            parse("[[[[あ]]]]").unwrap_err(),
            SentenceError::NestedTarget
        );
        assert_eq!(parse("[[]]").unwrap_err(), SentenceError::EmptyTarget);
    }
}
