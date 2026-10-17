//! Renders a parsed sentence into fragments for display.

use super::Segment;
use super::target::{pieces_base, pieces_kana};
use crate::{Fragment, SentenceDisplay};

pub fn render(segments: &[Segment], display: SentenceDisplay) -> Vec<Fragment> {
    segments
        .iter()
        .map(|segment| match segment {
            Segment::Text(text) => Fragment::Text { text: text.clone() },
            Segment::Ruby(ruby) => Fragment::Ruby {
                base: ruby.base.clone(),
                ruby: ruby.kana.clone(),
            },
            Segment::Target(pieces) => match display {
                SentenceDisplay::KanaOnly => Fragment::Target {
                    text: pieces_kana(pieces),
                },
                SentenceDisplay::Blank => Fragment::Blank,
                SentenceDisplay::KanjiOnly => Fragment::Target {
                    text: pieces_base(pieces),
                },
            },
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::marked::parse;

    fn rendered(display: SentenceDisplay) -> Vec<Fragment> {
        render(
            &parse("{今日|きょう}[[{諦|あきら}める]]。").unwrap(),
            display,
        )
    }

    fn head() -> Fragment {
        Fragment::Ruby {
            base: "今日".into(),
            ruby: "きょう".into(),
        }
    }

    fn tail() -> Fragment {
        Fragment::Text { text: "。".into() }
    }

    #[test]
    fn kana_only_shows_target_kana() {
        assert_eq!(
            rendered(SentenceDisplay::KanaOnly),
            vec![
                head(),
                Fragment::Target {
                    text: "あきらめる".into()
                },
                tail()
            ]
        );
    }

    #[test]
    fn blank_hides_target() {
        assert_eq!(
            rendered(SentenceDisplay::Blank),
            vec![head(), Fragment::Blank, tail()]
        );
    }

    #[test]
    fn kanji_only_shows_target_base() {
        assert_eq!(
            rendered(SentenceDisplay::KanjiOnly),
            vec![
                head(),
                Fragment::Target {
                    text: "諦める".into()
                },
                tail()
            ]
        );
    }
}
