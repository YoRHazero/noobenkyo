//! How a command ends.

/// A command that ran.
pub enum Outcome {
    Done,
    /// What the agent has to fix, one line of Chinese each.
    Problems(Vec<String>),
}

impl Outcome {
    /// Done when there are no problems.
    pub fn from_problems(problems: Vec<String>) -> Self {
        if problems.is_empty() {
            Outcome::Done
        } else {
            Outcome::Problems(problems)
        }
    }
}

/// Why a command could not run, in Chinese.
pub struct Failure(pub String);

impl<E: std::error::Error> From<E> for Failure {
    fn from(error: E) -> Self {
        Failure(error.to_string())
    }
}
