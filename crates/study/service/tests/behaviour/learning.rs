//! The learning line kept by the service.

use study::{Next, StudyService};

use crate::support::key;

#[test]
fn the_learning_zone_survives_reopening_until_its_capacity_changes() {
    let dir = tempfile::tempdir().unwrap();
    let mut study = StudyService::open(dir.path()).unwrap();
    study.learn().fill(vec![key("a"), key("b")]).unwrap();

    let mut reopened = StudyService::open(dir.path()).unwrap();
    assert!(matches!(reopened.learn().next(), Next::NeedContent(_)));

    let mut smaller = reopened.settings();
    smaller.learn.capacity = 1;
    reopened.update_settings(smaller).unwrap();
    assert_eq!(reopened.learn().next(), Next::Empty);
}
