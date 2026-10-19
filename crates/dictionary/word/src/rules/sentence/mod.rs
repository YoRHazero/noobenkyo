//! The marked text of sentences: `{漢字|かな}` is ruby and `[[…]]` marks
//! the target, which must appear exactly once.

mod check;
mod error;
mod fragment;
mod parse;
mod plain;
mod render;
mod segment;

pub use check::check;
pub use error::SentenceError;
pub use fragment::{Fragment, SentenceDisplay, TargetPiece};
pub use parse::parse;
pub use plain::plain_text;
pub use render::render;
pub use segment::{Piece, Ruby, Segment};

/// Parses and checks a marked sentence in one step.
pub fn parse_checked(marked: &str) -> Result<Vec<Segment>, SentenceError> {
    let segments = parse(marked)?;
    check(&segments)?;
    Ok(segments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_checks() {
        assert_eq!(parse_checked("[[{明日|あした}]]").unwrap().len(), 1);
        assert_eq!(
            parse_checked("明日").unwrap_err(),
            SentenceError::TargetCount(0)
        );
        assert_eq!(
            parse_checked("[[明日").unwrap_err(),
            SentenceError::UnclosedTarget
        );
    }
}
