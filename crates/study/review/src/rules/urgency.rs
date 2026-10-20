use chrono::{DateTime, FixedOffset};

const MILLISECONDS_PER_DAY: f64 = 86_400_000.0;

/// Days from `then` to `now`, in fractions; never negative.
pub fn elapsed_days(now: DateTime<FixedOffset>, then: DateTime<FixedOffset>) -> f64 {
    ((now - then).num_milliseconds() as f64 / MILLISECONDS_PER_DAY).max(0.0)
}

/// r: days since last modified over the expected interval. 1 or more is due.
pub fn urgency(days: f64, interval: f64) -> f64 {
    days / interval
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::now;
    use chrono::Duration;

    #[test]
    fn counts_fractional_days_and_ignores_the_future() {
        assert_eq!(elapsed_days(now(), now() - Duration::hours(36)), 1.5);
        assert_eq!(elapsed_days(now(), now() + Duration::days(1)), 0.0);
        assert_eq!(urgency(6.0, 4.0), 1.5);
    }
}
