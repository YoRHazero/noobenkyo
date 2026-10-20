use study::{ReviewService, StudyService};

use crate::command_error::CommandError;

/// The session in progress. Without one, only starting a session makes sense.
pub fn current(study: &mut StudyService) -> Result<&mut ReviewService, CommandError> {
    study
        .review()
        .ok_or_else(|| CommandError::new("当前没有进行中的复习"))
}
