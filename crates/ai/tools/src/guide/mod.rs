//! What an agent reads before writing: the format, how to write well, an
//! example, and the order to use the commands in. The text ships with the
//! program, so it always matches the checks of the same build.

/// How to write entries for the dictionary.
pub fn dictionary_guide() -> &'static str {
    include_str!("dictionary.md")
}

/// How to write a script for a listening episode.
pub fn listening_guide() -> &'static str {
    include_str!("listening.md")
}
