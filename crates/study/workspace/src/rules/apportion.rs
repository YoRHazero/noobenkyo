/// Splits `total` seats in proportion to `ratios` by largest remainder;
/// ties go to the lower index. All-zero ratios get nothing.
pub fn apportion(total: u32, ratios: &[u32]) -> Vec<u32> {
    let sum: f64 = ratios.iter().map(|r| f64::from(*r)).sum();
    if sum <= 0.0 {
        return vec![0; ratios.len()];
    }
    let exact: Vec<f64> = ratios
        .iter()
        .map(|r| f64::from(total) * f64::from(*r) / sum)
        .collect();
    let mut seats: Vec<u32> = exact.iter().map(|e| e.floor() as u32).collect();
    let given: u32 = seats.iter().sum();
    let mut by_remainder: Vec<usize> = (0..ratios.len()).collect();
    by_remainder.sort_by(|a, b| {
        let fraction = |i: usize| exact[i] - exact[i].floor();
        fraction(*b).total_cmp(&fraction(*a)).then(a.cmp(b))
    });
    for index in by_remainder
        .into_iter()
        .take(usize::try_from(total.saturating_sub(given)).unwrap_or(0))
    {
        seats[index] += 1;
    }
    seats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_proportionally() {
        assert_eq!(apportion(10, &[1, 1]), [5, 5]);
        assert_eq!(apportion(10, &[3, 1]), [8, 2]);
        assert_eq!(apportion(5, &[1, 1, 1]), [2, 2, 1]);
        assert_eq!(apportion(4, &[0, 1]), [0, 4]);
        assert_eq!(apportion(4, &[0, 0]), [0, 0]);
    }
}
