/// What `pick` needs to know about a machine.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Availability<'a> {
    pub busy: bool,
    /// The model of its executor, if it has a live one.
    pub loaded: Option<&'a str>,
}

/// Among the free candidates (indexes in order of preference), the first
/// that already has `model` loaded, else the first. None when all are busy.
pub(crate) fn pick(machines: &[Availability], candidates: &[usize], model: &str) -> Option<usize> {
    let mut free = candidates.iter().copied().filter(|&i| !machines[i].busy);
    let first = free.clone().next()?;
    Some(
        free.find(|&i| machines[i].loaded == Some(model))
            .unwrap_or(first),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const FREE: Availability = Availability {
        busy: false,
        loaded: None,
    };
    const BUSY: Availability = Availability {
        busy: true,
        loaded: None,
    };

    fn loaded(model: &str) -> Availability<'_> {
        Availability {
            busy: false,
            loaded: Some(model),
        }
    }

    #[test]
    fn prefers_a_machine_with_the_model_loaded() {
        let machines = [loaded("tts"), loaded("llm"), FREE];
        assert_eq!(pick(&machines, &[0, 1, 2], "llm"), Some(1));
    }

    #[test]
    fn otherwise_takes_the_first_free_one_even_if_it_has_another_model() {
        let machines = [BUSY, loaded("tts"), FREE];
        assert_eq!(pick(&machines, &[0, 1, 2], "llm"), Some(1));
    }

    #[test]
    fn a_busy_machine_does_not_count_even_with_the_model() {
        let machines = [
            Availability {
                busy: true,
                loaded: Some("llm"),
            },
            FREE,
        ];
        assert_eq!(pick(&machines, &[0, 1], "llm"), Some(1));
    }

    #[test]
    fn only_looks_at_the_candidates() {
        let machines = [loaded("llm"), FREE, FREE];
        assert_eq!(pick(&machines, &[1, 2], "llm"), Some(1));
        assert_eq!(pick(&machines, &[], "llm"), None);
    }

    #[test]
    fn none_when_every_candidate_is_busy() {
        let machines = [BUSY, FREE, BUSY];
        assert_eq!(pick(&machines, &[0, 2], "llm"), None);
    }
}
