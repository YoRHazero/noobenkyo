use rand::Rng;
use rand::seq::IndexedRandom;

use crate::{Recent, SenseKey};

/// Picks one sense at random, leaving out the last `count` asked. `None`
/// when every sense is left out.
pub fn pick_fresh<'a>(
    keys: &[&'a SenseKey],
    recent: &Recent,
    count: usize,
    rng: &mut impl Rng,
) -> Option<&'a SenseKey> {
    let fresh: Vec<&SenseKey> = keys
        .iter()
        .copied()
        .filter(|key| !recent.within(key, count))
        .collect();
    fresh.choose(rng).copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::fixtures::key;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn leaves_out_the_recent_ones() {
        let (a, b) = (key("a"), key("b"));
        let mut recent = Recent::default();
        recent.push(a.clone(), 2);
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..20 {
            assert_eq!(pick_fresh(&[&a, &b], &recent, 1, &mut rng), Some(&b));
        }
        assert_eq!(pick_fresh(&[&a], &recent, 1, &mut rng), None);
        assert_eq!(pick_fresh(&[&a], &recent, 0, &mut rng), Some(&a));
    }
}
