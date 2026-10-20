use crate::Progress;

/// What a quiz needs about a sense, taken from the dictionary when the quiz is made.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Content {
    /// All glosses, the primary gloss first.
    pub glosses: Vec<String>,
    pub sentences: Vec<Sentence>,
    pub progress: Progress,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sentence {
    pub marked: String,
    pub translation: Option<String>,
}
