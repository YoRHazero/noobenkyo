//! The marked-text format of sentences: `{漢字|かな}` is ruby and `[[…]]`
//! marks the target, which must appear exactly once.

mod check;
mod error;
mod parse;
mod plain;
mod render;
mod segment;
mod target;

pub use check::check;
pub use error::MarkedError;
pub use parse::parse;
pub use plain::plain_text;
pub use render::render;
pub use segment::{Piece, Ruby, Segment};
pub use target::{target_kana, target_pieces};

/// Parses and checks a marked sentence in one step.
pub fn parse_checked(marked: &str) -> Result<Vec<Segment>, MarkedError> {
    let segments = parse(marked)?;
    check(&segments)?;
    Ok(segments)
}
