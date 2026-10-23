//! The standard script JSON every source delivers: parsing it into a
//! `Script`, the check it must pass, and what the script preview shows. All
//! pure functions.
//!
//! ```json
//! { "title": "…", "summary": "…", "lines": [ { "speaker": "…", "ja": "…", "zh": "…" } ] }
//! ```

mod field;
mod line;
mod parse;
mod preview;

pub use parse::parse_script;
pub use preview::{PreviewLine, ScriptPreview, preview_script};

/// Why the text is not a script that passes, one line of Chinese each.
/// Empty means it passes.
pub fn check_script(text: &str) -> Vec<String> {
    parse_script(text).err().unwrap_or_default()
}
