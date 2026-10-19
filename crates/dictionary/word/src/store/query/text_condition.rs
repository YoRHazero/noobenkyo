//! Where a search text may appear, layer by layer.

use sea_orm::sea_query::{Expr, ExprTrait};
use sea_orm::{ColumnTrait, Condition};

use crate::entity::{reading, sense, word};
use crate::rules::kana::to_hiragana;

/// In the writing or the origin.
pub fn word_matches(text: &str) -> Condition {
    Condition::any()
        .add(word::Column::Writing.contains(text))
        .add(word::Column::Origin.contains(text))
}

/// In the word, or in the kana. Kana is stored as hiragana, so katakana in
/// the text is compared as hiragana.
pub fn reading_matches(text: &str) -> Condition {
    word_matches(text).add(reading::Column::Kana.contains(to_hiragana(text)))
}

/// In the reading, or in any gloss.
pub fn sense_matches(text: &str) -> Condition {
    reading_matches(text)
        .add(sense::Column::PrimaryGloss.contains(text))
        .add(Expr::col((sense::Entity, sense::Column::SecondaryGlosses)).like(format!("%{text}%")))
}
