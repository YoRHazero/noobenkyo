use chrono::{DateTime, FixedOffset};
use rand::Rng;

use super::group_quota::group_quotas;
use super::review_entry::select_review;
use crate::{Candidate, SenseKey, WorkspaceSettings};

/// Chooses a fixed group: seats per stage by ratio, then by weight within each stage.
pub fn select_group(
    candidates: &[&Candidate],
    settings: &WorkspaceSettings,
    now: DateTime<FixedOffset>,
    rng: &mut impl Rng,
) -> Vec<SenseKey> {
    let max_stage = settings.max_stage();
    let mut by_stage: Vec<Vec<&Candidate>> = vec![Vec::new(); settings.stage_intervals.len()];
    for candidate in candidates {
        let stage = usize::try_from(candidate.progress.stage.min(max_stage)).unwrap_or(0);
        by_stage[stage].push(candidate);
    }
    let available: Vec<u32> = by_stage
        .iter()
        .map(|s| u32::try_from(s.len()).unwrap_or(u32::MAX))
        .collect();
    let quotas = group_quotas(
        settings.fixed_group_size,
        &settings.group_stage_ratios,
        &available,
    );
    by_stage
        .iter()
        .zip(quotas)
        .flat_map(|(stage, quota)| {
            select_review(
                stage,
                usize::try_from(quota).unwrap_or(0),
                settings,
                now,
                rng,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::fixtures::{candidate, now, rng};

    #[test]
    fn fills_quotas_per_stage() {
        let settings = WorkspaceSettings {
            fixed_group_size: 4,
            stage_intervals: vec![1.0, 2.0],
            group_stage_ratios: vec![1.0, 1.0],
            ..WorkspaceSettings::default()
        };
        let all = [
            candidate("a", 0, 1, 1),
            candidate("b", 1, 1, 1),
            candidate("c", 1, 1, 1),
            candidate("d", 1, 1, 1),
            candidate("e", 9, 1, 1),
        ];
        let refs: Vec<&Candidate> = all.iter().collect();
        let group = select_group(&refs, &settings, now(), &mut rng());
        assert_eq!(group.len(), 4);
        assert!(group.iter().any(|k| k.writing == "a"));
    }
}
