//! Content rules of a parsed sentence: exactly one target, every kanji in
//! the target carries ruby, and ruby is reading kana. Kanji outside the
//! target may go without ruby.

use super::{Piece, Ruby, Segment, SentenceError};
use crate::rules::kana::is_reading_kana;
use crate::rules::kanji::is_kanji;

pub fn check(segments: &[Segment]) -> Result<(), SentenceError> {
    let targets = segments
        .iter()
        .filter(|s| matches!(s, Segment::Target(_)))
        .count();
    if targets != 1 {
        return Err(SentenceError::TargetCount(targets));
    }
    for segment in segments {
        match segment {
            Segment::Text(_) => {}
            Segment::Ruby(ruby) => check_ruby(ruby)?,
            Segment::Target(pieces) => {
                for piece in pieces {
                    match piece {
                        Piece::Text(text) => check_target_text(text)?,
                        Piece::Ruby(ruby) => check_ruby(ruby)?,
                    }
                }
            }
        }
    }
    Ok(())
}

fn check_target_text(text: &str) -> Result<(), SentenceError> {
    match text.chars().find(|c| is_kanji(*c)) {
        Some(c) => Err(SentenceError::TargetKanjiWithoutRuby(c)),
        None => Ok(()),
    }
}

fn check_ruby(ruby: &Ruby) -> Result<(), SentenceError> {
    if ruby.kana.chars().all(is_reading_kana) {
        Ok(())
    } else {
        Err(SentenceError::RubyNotKana(ruby.kana.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::sentence::parse;

    fn check_str(marked: &str) -> Result<(), SentenceError> {
        check(&parse(marked).unwrap())
    }

    #[test]
    fn accepts_kanji_without_ruby_outside_the_target() {
        assert_eq!(check_str("{何度|なんど}も[[{諦|あきら}めない]]。"), Ok(()));
        assert_eq!(check_str("何度も[[{諦|あきら}めない]]。"), Ok(()));
        assert_eq!(check_str("[[コーヒー]]を飲む"), Ok(()));
    }

    #[test]
    fn requires_exactly_one_target() {
        assert_eq!(
            check_str("あいう").unwrap_err(),
            SentenceError::TargetCount(0)
        );
        assert_eq!(
            check_str("[[あ]][[い]]").unwrap_err(),
            SentenceError::TargetCount(2)
        );
    }

    #[test]
    fn requires_ruby_on_target_kanji() {
        assert_eq!(
            check_str("[[諦めない]]").unwrap_err(),
            SentenceError::TargetKanjiWithoutRuby('諦')
        );
        assert_eq!(check_str("[[{諦|あきら}め{無|な}い]]"), Ok(()));
    }

    #[test]
    fn requires_reading_kana_in_ruby() {
        assert_eq!(
            check_str("[[{諦|abc}める]]").unwrap_err(),
            SentenceError::RubyNotKana("abc".into())
        );
        assert_eq!(
            check_str("{時々|ときゞ}[[{行|い}く]]").unwrap_err(),
            SentenceError::RubyNotKana("ときゞ".into())
        );
        assert_eq!(check_str("[[{珈琲|コーヒー}]]"), Ok(()));
    }
}
