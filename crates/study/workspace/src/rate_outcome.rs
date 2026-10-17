use crate::{AnswerRecord, Settlement};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RateOutcome {
    /// Present when the answer meets the logging condition.
    pub record: Option<AnswerRecord>,
    /// Present when the sense became full and must be settled, then removed.
    pub settlement: Option<Settlement>,
}
