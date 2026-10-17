use crate::Rating;

/// An answer is logged when it was rated vague or unknown, or when the
/// user overrode a prediction.
pub fn should_log(prediction: Option<Rating>, rating: Rating) -> bool {
    rating != Rating::Known || prediction.is_some_and(|predicted| predicted != rating)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logs_mistakes_and_overrides() {
        assert!(!should_log(None, Rating::Known));
        assert!(!should_log(Some(Rating::Known), Rating::Known));
        assert!(should_log(None, Rating::Vague));
        assert!(should_log(Some(Rating::Unknown), Rating::Unknown));
        assert!(should_log(Some(Rating::Unknown), Rating::Known));
    }
}
