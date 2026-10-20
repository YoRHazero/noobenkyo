use crate::{Quiz, SenseKey};

/// What a line needs to go on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Next {
    /// A quiz is in progress: show it again.
    Pending(Quiz),
    /// The content of this sense, to make the next quiz.
    NeedContent(SenseKey),
    /// Nothing left to ask.
    Empty,
}
