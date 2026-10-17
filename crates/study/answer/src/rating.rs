use serde::{Deserialize, Serialize};

/// A self-rating, or a prediction of one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum Rating {
    /// 会
    Known,
    /// 模糊
    Vague,
    /// 不会
    Unknown,
}
