/// The number a new episode gets: one more than the whole part of the
/// world's largest number, or 1 for its first episode.
pub fn next_number(largest: Option<f64>) -> f64 {
    largest.map_or(1.0, |largest| largest.floor() + 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follows_the_whole_part_of_the_largest() {
        assert_eq!(next_number(None), 1.0);
        assert_eq!(next_number(Some(3.0)), 4.0);
        assert_eq!(next_number(Some(3.5)), 4.0);
        assert_eq!(next_number(Some(-0.5)), 0.0);
    }
}
