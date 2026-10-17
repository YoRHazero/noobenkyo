use super::apportion::apportion;

/// Seats per stage for a fixed group: split by the stage ratios; a stage
/// short of senses passes its shortfall on to the others by ratio. If all
/// stages together fall short, the group is smaller.
pub fn group_quotas(size: u32, ratios: &[u32], available: &[u32]) -> Vec<u32> {
    let mut quotas = vec![0u32; ratios.len()];
    loop {
        let open: Vec<bool> = (0..ratios.len())
            .map(|s| ratios[s] > 0 && available.get(s).copied().unwrap_or(0) > quotas[s])
            .collect();
        let need = size - quotas.iter().sum::<u32>();
        if need == 0 || !open.contains(&true) {
            return quotas;
        }
        let open_ratios: Vec<u32> = (0..ratios.len())
            .map(|s| if open[s] { ratios[s] } else { 0 })
            .collect();
        let shares = apportion(need, &open_ratios);
        let mut progressed = false;
        for stage in 0..ratios.len() {
            let room = available.get(stage).copied().unwrap_or(0) - quotas[stage];
            let given = shares[stage].min(room);
            quotas[stage] += given;
            progressed |= given > 0;
        }
        if !progressed {
            return quotas;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_by_ratio_when_enough() {
        assert_eq!(group_quotas(10, &[3, 1], &[100, 100]), [8, 2]);
    }

    #[test]
    fn passes_shortfall_to_other_stages() {
        assert_eq!(group_quotas(10, &[1, 1, 1], &[1, 100, 100]), [1, 5, 4]);
    }

    #[test]
    fn shrinks_when_all_stages_fall_short() {
        assert_eq!(group_quotas(10, &[1, 1], &[2, 3]), [2, 3]);
    }

    #[test]
    fn zero_ratio_stages_get_nothing() {
        assert_eq!(group_quotas(4, &[0, 1], &[10, 2]), [0, 2]);
    }
}
