//! Content rules of a parsed sentence: exactly one target, every kanji
//! carries ruby, and ruby is kana.

use super::{MarkedError, Piece, Ruby, Segment};
use crate::rules::kana::is_kana;
use crate::rules::kanji::is_kanji;

pub fn check(segments: &[Segment]) -> Result<(), MarkedError> {
    let targets = segments
        .iter()
        .filter(|s| matches!(s, Segment::Target(_)))
        .count();
    if targets != 1 {
        return Err(MarkedError::TargetCount(targets));
    }
    for segment in segments {
        match segment {
            Segment::Text(text) => check_text(text)?,
            Segment::Ruby(ruby) => check_ruby(ruby)?,
            Segment::Target(pieces) => {
                for piece in pieces {
                    match piece {
                        Piece::Text(text) => check_text(text)?,
                        Piece::Ruby(ruby) => check_ruby(ruby)?,
                    }
                }
            }
        }
    }
    Ok(())
}

fn check_text(text: &str) -> Result<(), MarkedError> {
    match text.chars().find(|c| is_kanji(*c)) {
        Some(c) => Err(MarkedError::KanjiWithoutRuby(c)),
        None => Ok(()),
    }
}

fn check_ruby(ruby: &Ruby) -> Result<(), MarkedError> {
    if ruby.kana.chars().all(is_kana) {
        Ok(())
    } else {
        Err(MarkedError::RubyNotKana(ruby.kana.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::marked::parse;

    fn check_str(marked: &str) -> Result<(), MarkedError> {
        check(&parse(marked).unwrap())
    }

    #[test]
    fn accepts_valid_sentence() {
        assert_eq!(check_str("{何度|なんど}も[[{諦|あきら}めない]]。"), Ok(()));
        assert_eq!(
            check_str("[[コーヒー]]を飲む").unwrap_err(),
            MarkedError::KanjiWithoutRuby('飲')
        );
    }

    #[test]
    fn requires_exactly_one_target() {
        assert_eq!(
            check_str("あいう").unwrap_err(),
            MarkedError::TargetCount(0)
        );
        assert_eq!(
            check_str("[[あ]][[い]]").unwrap_err(),
            MarkedError::TargetCount(2)
        );
    }

    #[test]
    fn requires_ruby_on_target_kanji() {
        assert_eq!(
            check_str("[[諦めない]]").unwrap_err(),
            MarkedError::KanjiWithoutRuby('諦')
        );
    }

    #[test]
    fn requires_kana_ruby() {
        assert_eq!(
            check_str("[[{諦|abc}める]]").unwrap_err(),
            MarkedError::RubyNotKana("abc".into())
        );
    }
}
