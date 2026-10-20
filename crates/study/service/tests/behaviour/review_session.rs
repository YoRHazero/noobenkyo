//! Starting and ending a review session.

use study::{Next, SessionOptions, StudyError, StudyService};

use crate::support::candidate;

#[test]
fn a_session_lives_until_ended_and_ignores_new_settings() {
    let dir = tempfile::tempdir().unwrap();
    let mut study = StudyService::open(dir.path()).unwrap();
    assert!(study.review().is_none());
    study
        .start_review(vec![candidate("a")], SessionOptions::default())
        .unwrap();
    let mut settings = study.settings();
    settings.quiz.max_stage = 1;
    study.update_settings(settings).unwrap();
    assert!(matches!(
        study.review().unwrap().next(),
        Next::NeedContent(_)
    ));
    study.end_review();
    assert!(study.review().is_none());
}

#[test]
fn rejects_invalid_options() {
    let dir = tempfile::tempdir().unwrap();
    let mut study = StudyService::open(dir.path()).unwrap();
    let options = SessionOptions {
        start_steps: 0,
        ..SessionOptions::default()
    };
    assert!(matches!(
        study.start_review(vec![candidate("a")], options),
        Err(StudyError::InvalidOptions(_))
    ));
    assert!(study.review().is_none());
}
