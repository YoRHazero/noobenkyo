use rand::Rng;
use rand::seq::IndexedRandom;

use crate::{Candidate, LearningOrder, SenseKey};

/// Chooses up to `count` new senses, in creation order or at random.
pub fn select_learning(
    candidates: &[&Candidate],
    count: usize,
    order: LearningOrder,
    rng: &mut impl Rng,
) -> Vec<SenseKey> {
    match order {
        LearningOrder::Creation => candidates
            .iter()
            .take(count)
            .map(|c| c.key.clone())
            .collect(),
        LearningOrder::Random => candidates
            .choose_multiple(rng, count)
            .map(|c| c.key.clone())
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::fixtures::{candidate, key, rng};

    #[test]
    fn takes_creation_order_by_default() {
        let all = [
            candidate("a", 0, 0, 0),
            candidate("b", 0, 0, 0),
            candidate("c", 0, 0, 0),
        ];
        let refs: Vec<&Candidate> = all.iter().collect();
        assert_eq!(
            select_learning(&refs, 2, LearningOrder::Creation, &mut rng()),
            [key("a"), key("b")]
        );
    }

    #[test]
    fn random_order_takes_distinct_senses() {
        let all = [
            candidate("a", 0, 0, 0),
            candidate("b", 0, 0, 0),
            candidate("c", 0, 0, 0),
        ];
        let refs: Vec<&Candidate> = all.iter().collect();
        let chosen = select_learning(&refs, 5, LearningOrder::Random, &mut rng());
        assert_eq!(chosen.len(), 3);
    }
}
