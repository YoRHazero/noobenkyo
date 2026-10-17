//! The study group's public face: `StudyService` and `Settings`, plus the
//! sub-crate types that leave the group.

mod answers;
mod clock;
mod error;
mod exits;
mod questions;
mod service;
mod settings;
mod settings_file;
mod zone_entry;

pub use error::StudyError;
pub use service::StudyService;
pub use settings::Settings;

pub use study_answer::Rating;
pub use study_quiz::{
    AnswerInput, Card, ComprehensiveVisibility, QuestionType, QuizSettings, SentenceDisplay,
};
pub use study_workspace::{
    AnswerRecord, Candidate, LearningOrder, Progress, Question, QuestionView, RateOutcome,
    ReportedSentence, Reveal, RevealView, ReviewMode, SenseKey, Sentence, SentenceToRender,
    Settlement, Snapshot, WorkspaceSettings, Zone, ZoneStatus,
};
