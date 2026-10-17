use dictionary::Fragment;
use study::SentenceToRender;

use crate::command_error::CommandError;
use crate::convert::display;

/// Renders the sentence study chose, in the display study chose.
pub fn render(sentence: Option<SentenceToRender>) -> Result<Option<Vec<Fragment>>, CommandError> {
    let Some(sentence) = sentence else {
        return Ok(None);
    };
    let fragments =
        dictionary::render_sentence(&sentence.marked, display::to_dictionary(sentence.display))?;
    Ok(Some(fragments))
}
