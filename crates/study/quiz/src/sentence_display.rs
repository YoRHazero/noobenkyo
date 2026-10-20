/// How the target of a sentence is shown. Study's own type; core converts it
/// one-to-one to the dictionary's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SentenceDisplay {
    KanaOnly,
    Blank,
    KanjiOnly,
    /// Kanji with their ruby.
    Full,
}

/// A marked sentence for core to render.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SentenceToRender {
    pub marked: String,
    pub display: SentenceDisplay,
}
