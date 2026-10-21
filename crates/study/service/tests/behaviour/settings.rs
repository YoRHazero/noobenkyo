//! Reading and replacing the settings, and settings.json.

use study::{StudyService, StudySettings};

#[test]
fn rejects_invalid_settings_and_keeps_the_old() {
    let dir = tempfile::tempdir().unwrap();
    let mut study = StudyService::open(dir.path()).unwrap();
    let mut invalid = StudySettings::default();
    invalid.quiz.kanji_order.clear();
    assert!(study.update_settings(invalid).is_err());
    assert_eq!(study.settings(), StudySettings::default());

    let mut valid = StudySettings::default();
    valid.review.queue_limit = 30;
    study.update_settings(valid.clone()).unwrap();
    assert_eq!(StudyService::open(dir.path()).unwrap().settings(), valid);
}

#[test]
fn missing_fields_take_defaults() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("settings.json"),
        r#"{"review": {"queue_limit": 30}, "quiz": {"max_stage": 4}}"#,
    )
    .unwrap();
    let settings = StudyService::open(dir.path()).unwrap().settings();
    assert_eq!(settings.review.queue_limit, 30);
    assert_eq!(settings.review.interval_base, 1);
    assert_eq!(settings.quiz.max_stage, 4);
    assert_eq!(settings.learn, Default::default());
}

#[test]
fn invalid_or_unreadable_files_give_defaults() {
    for text in [r#"{"learn": {"capacity": 0}}"#, "not json"] {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("settings.json"), text).unwrap();
        let study = StudyService::open(dir.path()).unwrap();
        assert_eq!(study.settings(), StudySettings::default());
    }
}
