use crate::Rating;

/// Steps left after a self-rating: known takes one step; vague adds one, up
/// to the whole order; unknown starts the order over.
pub fn after_rating(remaining: u32, rating: Rating, steps: u32) -> u32 {
    match rating {
        Rating::Known => remaining.saturating_sub(1),
        Rating::Vague => (remaining + 1).min(steps),
        Rating::Unknown => steps,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_steps_by_rating() {
        assert_eq!(after_rating(2, Rating::Known, 4), 1);
        assert_eq!(after_rating(1, Rating::Known, 4), 0);
        assert_eq!(after_rating(2, Rating::Vague, 4), 3);
        assert_eq!(after_rating(4, Rating::Vague, 4), 4);
        assert_eq!(after_rating(1, Rating::Unknown, 4), 4);
    }
}
