use chrono::{DateTime, FixedOffset};
use rand::Rng;

use super::elapsed::elapsed_days;
use super::sample::weighted_sample;
use super::weight::review_weight;
use crate::{Candidate, SenseKey, WorkspaceSettings};

/// Draws up to `count` senses by review weight.
pub fn select_review(
    candidates: &[&Candidate],
    count: usize,
    settings: &WorkspaceSettings,
    now: DateTime<FixedOffset>,
    rng: &mut impl Rng,
) -> Vec<SenseKey> {
    let weighted = candidates
        .iter()
        .map(|c| {
            let days = elapsed_days(now, c.modified_at);
            (c.key.clone(), review_weight(days, &c.progress, settings))
        })
        .collect();
    weighted_sample(weighted, count, rng)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::fixtures::{candidate, key, now, rng};

    #[test]
    fn prefers_overdue_senses() {
        let all = [candidate("fresh", 3, 3, 0), candidate("overdue", 0, 1, 30)];
        let refs: Vec<&Candidate> = all.iter().collect();
        let mut rng = rng();
        let settings = WorkspaceSettings::default();
        let overdue = (0..200)
            .filter(|_| select_review(&refs, 1, &settings, now(), &mut rng) == [key("overdue")])
            .count();
        assert!(overdue > 190, "{overdue}");
    }
}
