use chrono::{DateTime, FixedOffset};

const MILLISECONDS_PER_DAY: f64 = 86_400_000.0;

/// Days from `then` to `now`, never negative.
pub fn elapsed_days(now: DateTime<FixedOffset>, then: DateTime<FixedOffset>) -> f64 {
    ((now - then).num_milliseconds() as f64 / MILLISECONDS_PER_DAY).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::fixtures::now;
    use chrono::Duration;

    #[test]
    fn counts_fractional_days() {
        assert_eq!(elapsed_days(now(), now() - Duration::hours(36)), 1.5);
        assert_eq!(elapsed_days(now(), now() + Duration::days(1)), 0.0);
    }
}
