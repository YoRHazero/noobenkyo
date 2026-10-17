use serde::{Deserialize, Serialize};

/// How the target of a sentence is shown. Non-target kanji always carry ruby.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum SentenceDisplay {
    /// The target shows only its kana.
    KanaOnly,
    /// The target is left blank.
    Blank,
    /// The target shows only its written form, without ruby.
    KanjiOnly,
}

/// One piece of a rendered sentence, handed to the frontend as is.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Fragment {
    Text { text: String },
    Ruby { base: String, ruby: String },
    Target { text: String },
    Blank,
}
