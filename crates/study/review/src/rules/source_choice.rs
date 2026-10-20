use rand::Rng;

/// Whether to ask from the queue rather than the pool: with probability
/// queue length ÷ limit, so a full queue is always asked from.
pub fn prefers_queue(queue_len: usize, limit: u32, rng: &mut impl Rng) -> bool {
    let share = queue_len as f64 / f64::from(limit.max(1));
    rng.random::<f64>() < share
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::rng;

    #[test]
    fn grows_with_the_queue() {
        let mut rng = rng();
        assert!((0..100).all(|_| !prefers_queue(0, 10, &mut rng)));
        assert!((0..100).all(|_| prefers_queue(10, 10, &mut rng)));
        let some = (0..1000).filter(|_| prefers_queue(3, 10, &mut rng)).count();
        assert!((250..350).contains(&some), "{some}");
    }
}
