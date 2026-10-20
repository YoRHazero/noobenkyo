use std::collections::HashSet;

use rand::Rng;
use rand::seq::SliceRandom;
use study_quiz::rules::order::steps_for;
use study_quiz::{QuizSettings, SenseKey};

use crate::{Item, LearnOrder};

/// New items from the candidates: those not in the zone yet, in the given
/// order, at most `room` of them. Each starts with the whole question order
/// to go.
pub fn entering(
    present: &[Item],
    candidates: Vec<SenseKey>,
    room: usize,
    order: LearnOrder,
    quiz: &QuizSettings,
    rng: &mut impl Rng,
) -> Vec<Item> {
    let mut seen: HashSet<SenseKey> = present.iter().map(|item| item.key.clone()).collect();
    let mut fresh: Vec<SenseKey> = candidates
        .into_iter()
        .filter(|key| seen.insert(key.clone()))
        .collect();
    if order == LearnOrder::Random {
        fresh.shuffle(rng);
    }
    fresh
        .into_iter()
        .take(room)
        .map(|key| Item {
            remaining: steps_for(quiz, &key.writing),
            key,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{key, rng};

    fn keys(items: &[Item]) -> Vec<&str> {
        items.iter().map(|item| item.key.writing.as_str()).collect()
    }

    #[test]
    fn takes_new_senses_in_creation_order() {
        let present = [Item {
            key: key("諦める"),
            remaining: 2,
        }];
        let candidates = vec![
            key("諦める"),
            key("きれい"),
            key("明日"),
            key("きれい"),
            key("上"),
        ];
        let items = entering(
            &present,
            candidates,
            2,
            LearnOrder::Creation,
            &QuizSettings::default(),
            &mut rng(),
        );
        assert_eq!(keys(&items), ["きれい", "明日"]);
        assert_eq!((items[0].remaining, items[1].remaining), (3, 4));
    }

    #[test]
    fn random_order_takes_distinct_senses() {
        let candidates = vec![key("a"), key("b"), key("c"), key("d")];
        let items = entering(
            &[],
            candidates,
            3,
            LearnOrder::Random,
            &QuizSettings::default(),
            &mut rng(),
        );
        let mut writings = keys(&items);
        writings.sort_unstable();
        writings.dedup();
        assert_eq!(writings.len(), 3);
    }
}
