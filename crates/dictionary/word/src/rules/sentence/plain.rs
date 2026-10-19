use super::{Piece, Segment};

/// The sentence without marks: ruby keeps only its base.
pub fn plain_text(segments: &[Segment]) -> String {
    let mut text = String::new();
    for segment in segments {
        match segment {
            Segment::Text(t) => text.push_str(t),
            Segment::Ruby(ruby) => text.push_str(&ruby.base),
            Segment::Target(pieces) => {
                for piece in pieces {
                    match piece {
                        Piece::Text(t) => text.push_str(t),
                        Piece::Ruby(ruby) => text.push_str(&ruby.base),
                    }
                }
            }
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::sentence::parse;

    #[test]
    fn strips_marks() {
        let segments = parse("{何度|なんど}も、[[{諦|あきら}めない]]でね。").unwrap();
        assert_eq!(plain_text(&segments), "何度も、諦めないでね。");
    }
}
