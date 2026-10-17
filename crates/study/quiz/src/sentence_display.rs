/// How the target of the sentence is shown. Study's own type; core converts
/// it one-to-one to the dictionary's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SentenceDisplay {
    KanaOnly,
    Blank,
    KanjiOnly,
}
