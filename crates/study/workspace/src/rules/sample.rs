use rand::Rng;

/// Draws up to `count` distinct items, each with probability proportional to
/// its positive weight (Efraimidis–Spirakis).
pub fn weighted_sample<T>(weighted: Vec<(T, f64)>, count: usize, rng: &mut impl Rng) -> Vec<T> {
    let mut keyed: Vec<(f64, T)> = weighted
        .into_iter()
        .map(|(item, weight)| {
            let uniform: f64 = 1.0 - rng.random::<f64>();
            (uniform.ln() / weight, item)
        })
        .collect();
    keyed.sort_by(|a, b| b.0.total_cmp(&a.0));
    keyed
        .into_iter()
        .take(count)
        .map(|(_, item)| item)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::fixtures::rng;

    #[test]
    fn draws_distinct_items_up_to_count() {
        let mut rng = rng();
        let drawn = weighted_sample(vec![("a", 1.0), ("b", 1.0), ("c", 1.0)], 2, &mut rng);
        assert_eq!(drawn.len(), 2);
        assert_ne!(drawn[0], drawn[1]);
        assert_eq!(weighted_sample(vec![("a", 1.0)], 5, &mut rng), ["a"]);
    }

    #[test]
    fn favors_heavy_items() {
        let mut rng = rng();
        let heavy = (0..1000)
            .filter(|_| {
                weighted_sample(vec![("light", 1.0), ("heavy", 9.0)], 1, &mut rng)[0] == "heavy"
            })
            .count();
        assert!((850..950).contains(&heavy), "{heavy}");
    }
}
