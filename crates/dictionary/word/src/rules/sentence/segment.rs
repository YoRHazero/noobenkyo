/// A top-level piece of a parsed sentence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Segment {
    Text(String),
    Ruby(Ruby),
    Target(Vec<Piece>),
}

/// A piece inside the target.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Piece {
    Text(String),
    Ruby(Ruby),
}

/// Ruby: a written base with its kana.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ruby {
    pub base: String,
    pub kana: String,
}
