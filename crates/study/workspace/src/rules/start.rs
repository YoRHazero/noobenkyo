use crate::Zone;

/// Steps left when a sense enters: the whole order in learning, one in review.
pub fn start_remaining(zone: Zone, steps: u32) -> u32 {
    match zone {
        Zone::Learning => steps,
        Zone::Review => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn learning_starts_from_the_whole_order() {
        assert_eq!(start_remaining(Zone::Learning, 4), 4);
        assert_eq!(start_remaining(Zone::Review, 4), 1);
    }
}
