//! Topping the zone up.

use std::collections::HashSet;

use study_quiz::Next;

use crate::support::{key, open};

#[test]
fn fills_up_to_capacity_without_duplicates() {
    let dir = tempfile::tempdir().unwrap();
    let mut learn = open(dir.path(), 2);
    assert!(!learn.is_full());
    learn.fill(vec![key("a")]).unwrap();
    assert!(!learn.is_full());
    learn.fill(vec![key("a"), key("b"), key("c")]).unwrap();
    assert!(learn.is_full());

    let mut asked = HashSet::new();
    for _ in 0..20 {
        let Next::NeedContent(key) = learn.next() else {
            panic!("expected a sense to ask");
        };
        asked.insert(key.writing);
    }
    assert_eq!(asked, HashSet::from(["a".to_owned(), "b".to_owned()]));
}
