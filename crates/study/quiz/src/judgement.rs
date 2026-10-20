use serde::Serialize;

/// Whether a submitted answer is right, as far as a fixed rule can tell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, specta::Type)]
pub enum Judgement {
    Correct,
    Wrong,
    /// The rule can tell neither; a model may judge it instead.
    Unsure,
}
