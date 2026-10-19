//! Entry text → content the dictionary understands. Checking and
//! converting are one step: a rule fails exactly where its value would be
//! taken, and that is where the problem is recorded.

mod convert_text;
mod converted;
mod entry_content;
mod entry_error;
mod reading_layer;
mod sense_layer;
mod sentences;
mod word_layer;

pub use convert_text::convert;
pub use converted::{Converted, ConvertedEntry};
