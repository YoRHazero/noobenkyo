//! Renders a parsed sentence into fragments for display.

use super::{Fragment, Piece, Segment, SentenceDisplay, TargetPiece};

pub fn render(segments: &[Segment], display: SentenceDisplay) -> Vec<Fragment> {
    segments
        .iter()
        .map(|segment| match segment {
            Segment::Text(text) => Fragment::Text { text: text.clone() },
            Segment::Ruby(ruby) => Fragment::Ruby {
                base: ruby.base.clone(),
                ruby: ruby.kana.clone(),
            },
            Segment::Target(pieces) => render_target(pieces, display),
        })
        .collect()
}

fn render_target(pieces: &[Piece], display: SentenceDisplay) -> Fragment {
    let pieces = match display {
        SentenceDisplay::Blank => return Fragment::Blank,
        SentenceDisplay::KanaOnly => vec![TargetPiece::Text {
            text: joined(pieces, |text| text, |ruby| &ruby.kana),
        }],
        SentenceDisplay::KanjiOnly => vec![TargetPiece::Text {
            text: joined(pieces, |text| text, |ruby| &ruby.base),
        }],
        SentenceDisplay::Full => pieces
            .iter()
            .map(|piece| match piece {
                Piece::Text(text) => TargetPiece::Text { text: text.clone() },
                Piece::Ruby(ruby) => TargetPiece::Ruby {
                    base: ruby.base.clone(),
                    ruby: ruby.kana.clone(),
                },
            })
            .collect(),
    };
    Fragment::Target { pieces }
}

/// The target as one string: text pieces as they are, ruby pieces through `ruby_part`.
fn joined(
    pieces: &[Piece],
    text_part: impl Fn(&String) -> &String,
    ruby_part: impl Fn(&super::Ruby) -> &String,
) -> String {
    pieces
        .iter()
        .map(|piece| match piece {
            Piece::Text(text) => text_part(text).as_str(),
            Piece::Ruby(ruby) => ruby_part(ruby).as_str(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::sentence::parse;

    fn rendered(display: SentenceDisplay) -> Vec<Fragment> {
        render(&parse("今日[[{諦|あきら}める]]。").unwrap(), display)
    }

    fn target_text(text: &str) -> Fragment {
        Fragment::Target {
            pieces: vec![TargetPiece::Text { text: text.into() }],
        }
    }

    fn head() -> Fragment {
        Fragment::Text {
            text: "今日".into(),
        }
    }

    fn tail() -> Fragment {
        Fragment::Text { text: "。".into() }
    }

    #[test]
    fn kana_only_shows_target_kana() {
        assert_eq!(
            rendered(SentenceDisplay::KanaOnly),
            vec![head(), target_text("あきらめる"), tail()]
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
            vec![head(), target_text("諦める"), tail()]
        );
    }

    #[test]
    fn full_keeps_target_ruby() {
        assert_eq!(
            rendered(SentenceDisplay::Full),
            vec![
                head(),
                Fragment::Target {
                    pieces: vec![
                        TargetPiece::Ruby {
                            base: "諦".into(),
                            ruby: "あきら".into()
                        },
                        TargetPiece::Text {
                            text: "める".into()
                        },
                    ]
                },
                tail()
            ]
        );
    }

    #[test]
    fn keeps_ruby_outside_the_target() {
        let fragments = render(
            &parse("{夢|ゆめ}を[[{諦|あきら}めた]]").unwrap(),
            SentenceDisplay::KanjiOnly,
        );
        assert_eq!(
            fragments[0],
            Fragment::Ruby {
                base: "夢".into(),
                ruby: "ゆめ".into()
            }
        );
    }
}
