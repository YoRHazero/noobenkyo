use dictionary::Fragment;
use study::SentenceToRender;

use crate::command_error::CommandError;
use crate::convert::display;

/// Renders the sentence study chose, in the display study chose.
pub fn render(sentence: SentenceToRender) -> Result<Vec<Fragment>, CommandError> {
    Ok(dictionary::render_sentence(
        &sentence.marked,
        display::to_dictionary(sentence.display),
    )?)
}
