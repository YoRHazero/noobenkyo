/// k: how many recently asked senses are excluded, the smaller of the limit
/// and one less than the zone's size.
pub fn recent_window(limit: u32, zone_size: usize) -> usize {
    usize::try_from(limit)
        .unwrap_or(usize::MAX)
        .min(zone_size.saturating_sub(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shrinks_with_small_zones() {
        assert_eq!(recent_window(2, 10), 2);
        assert_eq!(recent_window(2, 2), 1);
        assert_eq!(recent_window(2, 1), 0);
        assert_eq!(recent_window(2, 0), 0);
    }
}
