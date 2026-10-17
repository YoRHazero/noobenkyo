use rand::Rng;

/// Picks an index with probability proportional to its weight; `None` if all weights are zero.
pub fn pick_index(weights: &[u32], rng: &mut impl Rng) -> Option<usize> {
    let total: u64 = weights.iter().map(|w| u64::from(*w)).sum();
    if total == 0 {
        return None;
    }
    let mut roll = rng.random_range(0..total);
    for (index, weight) in weights.iter().enumerate() {
        let weight = u64::from(*weight);
        if roll < weight {
            return Some(index);
        }
        roll -= weight;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::fixtures::rng;

    #[test]
    fn picks_by_weight() {
        let mut rng = rng();
        assert_eq!(pick_index(&[0, 3, 0], &mut rng), Some(1));
        assert_eq!(pick_index(&[0, 0], &mut rng), None);
        assert_eq!(pick_index(&[], &mut rng), None);
        let ones = (0..1000)
            .filter(|_| pick_index(&[1, 3], &mut rng) == Some(1))
            .count();
        assert!((700..800).contains(&ones), "{ones}");
    }
}
