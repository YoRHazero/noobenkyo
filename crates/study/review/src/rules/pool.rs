use chrono::{DateTime, FixedOffset};
use rand::Rng;
use study_quiz::SenseKey;

use super::urgency::{elapsed_days, urgency};
use crate::{Candidate, PickOrder, SessionOptions};

/// The pool of a session: candidates below stage N, ordered by urgency, only
/// the first `size` of them for a group. Ties keep the candidates' order.
pub fn make_pool(
    candidates: Vec<Candidate>,
    options: &SessionOptions,
    max_stage: u32,
    interval: impl Fn(&Candidate) -> f64,
    now: DateTime<FixedOffset>,
    rng: &mut impl Rng,
) -> Vec<SenseKey> {
    let mut keyed: Vec<(f64, SenseKey)> = candidates
        .into_iter()
        .filter(|c| c.stage < max_stage)
        .map(|c| {
            let r = urgency(elapsed_days(now, c.modified_at), interval(&c));
            let sort_key = match options.order {
                PickOrder::Urgent => r,
                // Drawn ahead with probability proportional to r (Efraimidis–Spirakis).
                PickOrder::Weighted if r > 0.0 => rng.random::<f64>().powf(1.0 / r),
                PickOrder::Weighted => 0.0,
            };
            (sort_key, c.key)
        })
        .collect();
    keyed.sort_by(|a, b| b.0.total_cmp(&a.0));
    let size = options
        .size
        .map_or(usize::MAX, |s| usize::try_from(s).unwrap_or(usize::MAX));
    keyed.into_iter().take(size).map(|(_, key)| key).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{candidate, now, rng};

    fn interval(c: &Candidate) -> f64 {
        f64::from(3 * c.stage + 1)
    }

    fn writings(pool: &[SenseKey]) -> Vec<&str> {
        pool.iter().map(|k| k.writing.as_str()).collect()
    }

    fn options(size: Option<u32>, order: PickOrder) -> SessionOptions {
        SessionOptions {
            size,
            start_steps: 1,
            order,
        }
    }

    #[test]
    fn most_overdue_first_and_mastered_left_out() {
        // r: A 3/1 = 3, B 7/7 = 1, C 14/7 = 2, D 0, M is mastered.
        let candidates = vec![
            candidate("B", 2, 7),
            candidate("A", 0, 3),
            candidate("M", 7, 300),
            candidate("D", 1, 0),
            candidate("C", 2, 14),
        ];
        let pool = make_pool(
            candidates.clone(),
            &options(None, PickOrder::Urgent),
            7,
            interval,
            now(),
            &mut rng(),
        );
        assert_eq!(writings(&pool), ["A", "C", "B", "D"]);
        let group = make_pool(
            candidates,
            &options(Some(2), PickOrder::Urgent),
            7,
            interval,
            now(),
            &mut rng(),
        );
        assert_eq!(writings(&group), ["A", "C"]);
    }

    #[test]
    fn weighted_order_favours_the_overdue() {
        // r: heavy 9, light 1, fresh 0.
        let candidates = vec![
            candidate("light", 0, 1),
            candidate("heavy", 0, 9),
            candidate("fresh", 0, 0),
        ];
        let mut rng = rng();
        let mut heavy_first = 0;
        for _ in 0..1000 {
            let pool = make_pool(
                candidates.clone(),
                &options(None, PickOrder::Weighted),
                7,
                interval,
                now(),
                &mut rng,
            );
            assert_eq!(pool[2].writing, "fresh");
            heavy_first += usize::from(pool[0].writing == "heavy");
        }
        assert!((850..950).contains(&heavy_first), "{heavy_first}");
    }
}
