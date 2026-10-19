use dictionary_word::rules::sentence;

use crate::{DictionaryError, Fragment, SentenceDisplay};

/// Renders a marked sentence into fragments for the given display.
pub fn render_sentence(
    marked: &str,
    display: SentenceDisplay,
) -> Result<Vec<Fragment>, DictionaryError> {
    let segments = sentence::parse_checked(marked)?;
    Ok(sentence::render(&segments, display))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TargetPiece;

    #[test]
    fn renders_valid_and_rejects_invalid_sentences() {
        let fragments = render_sentence("今日[[{諦|あきら}める]]", SentenceDisplay::Full).unwrap();
        assert_eq!(
            fragments,
            [
                Fragment::Text {
                    text: "今日".into()
                },
                Fragment::Target {
                    pieces: vec![
                        TargetPiece::Ruby {
                            base: "諦".into(),
                            ruby: "あきら".into()
                        },
                        TargetPiece::Text {
                            text: "める".into()
                        }
                    ]
                }
            ]
        );
        assert!(render_sentence("[[諦める]]", SentenceDisplay::Blank).is_err());
    }
}
