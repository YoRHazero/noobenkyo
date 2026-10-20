use serde::Serialize;

/// The input a quiz asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum AnswerInput {
    /// No input: self-rating only.
    None,
    Chinese,
    /// Kana, optionally limited to a number of kana.
    Kana {
        max_length: Option<u32>,
    },
}
