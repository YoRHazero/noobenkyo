//! What core hands the frontend beyond the features' own types: a shell
//! around a feature's value and the sentences rendered for it, one type per
//! file.

pub mod quiz_payload;
pub mod reveal_payload;
pub mod sense_detail_payload;

pub use quiz_payload::QuizPayload;
pub use reveal_payload::RevealPayload;
pub use sense_detail_payload::SenseDetailPayload;
