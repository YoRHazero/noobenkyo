use serde::{Deserialize, Serialize};

/// How the target of a sentence is shown. Ruby outside the target is always shown.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum SentenceDisplay {
    /// The target shows only its kana.
    KanaOnly,
    /// The target is left blank.
    Blank,
    /// The target shows only its written form, without ruby.
    KanjiOnly,
    /// The target shows its written form with its ruby.
    Full,
}

/// One piece of a rendered sentence, handed to the frontend as is.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Fragment {
    Text {
        text: String,
    },
    Ruby {
        base: String,
        ruby: String,
    },
    /// The target as one unit, made of pieces with or without ruby.
    Target {
        pieces: Vec<TargetPiece>,
    },
    Blank,
}

/// A piece of a rendered target.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum TargetPiece {
    Text { text: String },
    Ruby { base: String, ruby: String },
}
