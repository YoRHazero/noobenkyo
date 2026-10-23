//! The standard script JSON every source delivers: parsing it into a
//! `Script`, and the check it must pass. All pure functions.
//!
//! ```json
//! { "title": "…", "summary": "…", "lines": [ { "speaker": "…", "ja": "…", "zh": "…" } ] }
//! ```

mod field;
mod line;
mod parse;

pub use parse::parse_script;

/// Why the text is not a script that passes, one line of Chinese each.
/// Empty means it passes.
pub fn check_script(text: &str) -> Vec<String> {
    parse_script(text).err().unwrap_or_default()
}
