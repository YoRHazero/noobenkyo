use dictionary_word::rules::marked;

use crate::{DictionaryError, Fragment, SentenceDisplay};

/// Renders a marked sentence into fragments for the given display.
pub fn render_sentence(
    marked_text: &str,
    display: SentenceDisplay,
) -> Result<Vec<Fragment>, DictionaryError> {
    let segments = marked::parse_checked(marked_text)?;
    Ok(marked::render(&segments, display))
}
