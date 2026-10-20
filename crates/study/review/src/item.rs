use study_quiz::SenseKey;

/// A sense in the queue, practised until full.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub key: SenseKey,
    /// Steps left until full.
    pub remaining: u32,
}
