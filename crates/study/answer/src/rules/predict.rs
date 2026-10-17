use super::kana_input::normalize_kana_input;
use crate::Rating;

/// Predicts the self-rating: known if the kana matches the expected answer,
/// unknown otherwise; no prediction when nothing is expected.
pub fn predict(input: &str, expected: Option<&str>) -> Option<Rating> {
    let expected = expected?;
    if normalize_kana_input(input) == expected {
        Some(Rating::Known)
    } else {
        Some(Rating::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_normalized_kana() {
        assert_eq!(
            predict("アキラメル", Some("あきらめる")),
            Some(Rating::Known)
        );
        assert_eq!(predict("やめる", Some("あきらめる")), Some(Rating::Unknown));
        assert_eq!(predict("放弃", None), None);
    }
}
