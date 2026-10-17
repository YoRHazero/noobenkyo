use super::{Piece, Segment};

/// Pieces of the first target, if any.
pub fn target_pieces(segments: &[Segment]) -> Option<&[Piece]> {
    segments.iter().find_map(|segment| match segment {
        Segment::Target(pieces) => Some(pieces.as_slice()),
        _ => None,
    })
}

/// Kana of the target as written in the sentence: ruby contributes its kana.
pub fn target_kana(segments: &[Segment]) -> Option<String> {
    target_pieces(segments).map(pieces_kana)
}

pub(super) fn pieces_kana(pieces: &[Piece]) -> String {
    pieces
        .iter()
        .map(|piece| match piece {
            Piece::Text(text) => text.as_str(),
            Piece::Ruby(ruby) => ruby.kana.as_str(),
        })
        .collect()
}

pub(super) fn pieces_base(pieces: &[Piece]) -> String {
    pieces
        .iter()
        .map(|piece| match piece {
            Piece::Text(text) => text.as_str(),
            Piece::Ruby(ruby) => ruby.base.as_str(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::marked::parse;

    #[test]
    fn reads_target_kana() {
        let segments = parse("{何度|なんど}も[[{諦|あきら}めない]]。").unwrap();
        assert_eq!(target_kana(&segments).as_deref(), Some("あきらめない"));
        assert_eq!(target_kana(&parse("なし").unwrap()), None);
    }
}
