use dictionary_word::rules::kana::normalize_reading;
use dictionary_word::rules::kanji::has_kanji;

use super::entry_error::EntryError;

/// The kana of the reading layer, or `None` when the entry stops above it.
/// A word with kanji takes the given kana; a word without kanji derives it
/// from the writing, and given kana must match. `needed` is whether layers
/// below ask for a reading.
pub fn kana(
    writing: &str,
    given: Option<&str>,
    needed: bool,
) -> Result<Option<String>, EntryError> {
    if has_kanji(writing) {
        return match given {
            Some(raw) => Ok(Some(normalize_reading(raw)?)),
            None if needed => Err(EntryError::MissingKana),
            None => Ok(None),
        };
    }
    match given {
        Some(raw) => {
            let kana = normalize_reading(raw)?;
            let expected = normalize_reading(writing)?;
            if kana == expected {
                Ok(Some(kana))
            } else {
                Err(EntryError::KanaMismatch { expected })
            }
        }
        None if needed => Ok(Some(normalize_reading(writing)?)),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dictionary_word::rules::kana::KanaError;

    #[test]
    fn kanji_word_takes_given_kana() {
        assert_eq!(kana("明日", Some("アス"), false), Ok(Some("あす".into())));
        assert_eq!(kana("明日", None, false), Ok(None));
        assert_eq!(kana("明日", None, true), Err(EntryError::MissingKana));
    }

    #[test]
    fn kana_word_derives_kana_from_writing() {
        assert_eq!(kana("コーヒー", None, true), Ok(Some("こーひー".into())));
        assert_eq!(kana("コーヒー", None, false), Ok(None));
        assert_eq!(
            kana("きれい", Some("キレイ"), false),
            Ok(Some("きれい".into()))
        );
        assert_eq!(
            kana("きれい", Some("きれ"), false),
            Err(EntryError::KanaMismatch {
                expected: "きれい".into()
            })
        );
    }

    #[test]
    fn derived_kana_follows_the_reading_rule() {
        assert_eq!(
            kana("いすゞ", None, true),
            Err(EntryError::Kana(KanaError::NotKana('ゞ')))
        );
    }
}
