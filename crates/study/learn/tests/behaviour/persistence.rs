//! learning.json across restarts, and settings that empty the zone.

use study_learn::LearnSettings;
use study_quiz::{Next, QuestionType, QuizSettings, Rating};

use crate::support::{answer, ask, key, open};

#[test]
fn reopening_keeps_the_zone_and_its_steps() {
    let dir = tempfile::tempdir().unwrap();
    let mut learn = open(dir.path(), 1);
    learn.fill(vec![key("諦める")]).unwrap();
    answer(&mut learn, Rating::Known);
    let mut reopened = open(dir.path(), 1);
    assert!(reopened.is_full());
    assert_eq!(ask(&mut reopened).view.remaining, 3);
}

#[test]
fn an_unreadable_file_gives_an_empty_zone() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("learning.json"), r#"{"old": true}"#).unwrap();
    let mut learn = open(dir.path(), 1);
    assert_eq!(learn.next(), Next::Empty);
}

#[test]
fn changing_capacity_or_order_empties_the_zone() {
    let dir = tempfile::tempdir().unwrap();
    let mut learn = open(dir.path(), 2);
    learn.fill(vec![key("諦める")]).unwrap();

    let calmer = LearnSettings {
        capacity: 2,
        recent_limit: 0,
        ..LearnSettings::default()
    };
    learn
        .apply_settings(QuizSettings::default(), calmer.clone())
        .unwrap();
    assert_eq!(learn.next(), Next::NeedContent(key("諦める")));

    let shorter = QuizSettings {
        kanji_order: vec![QuestionType::Comprehensive],
        ..QuizSettings::default()
    };
    learn.apply_settings(shorter, calmer).unwrap();
    assert_eq!(learn.next(), Next::Empty);
    assert_eq!(open(dir.path(), 2).next(), Next::Empty);
}
