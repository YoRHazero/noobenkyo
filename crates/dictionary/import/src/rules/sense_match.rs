/// Where incoming glosses belong among the senses of a reading.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SenseMatch {
    /// No gloss in common with any sense.
    New,
    /// Glosses in common with exactly this sense (index into the input).
    Existing(usize),
    /// Glosses in common with several senses.
    Conflict(Vec<usize>),
}

/// Matches by gloss-list intersection.
pub fn match_sense(existing: &[Vec<String>], incoming: &[String]) -> SenseMatch {
    let hits: Vec<usize> = existing
        .iter()
        .enumerate()
        .filter(|(_, glosses)| glosses.iter().any(|gloss| incoming.contains(gloss)))
        .map(|(index, _)| index)
        .collect();
    match hits.as_slice() {
        [] => SenseMatch::New,
        [only] => SenseMatch::Existing(*only),
        _ => SenseMatch::Conflict(hits),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn matches_by_intersection() {
        let existing = vec![list(&["放弃", "死心"]), list(&["明天"])];
        assert_eq!(match_sense(&existing, &list(&["咖啡"])), SenseMatch::New);
        assert_eq!(
            match_sense(&existing, &list(&["断念", "死心"])),
            SenseMatch::Existing(0)
        );
        assert_eq!(
            match_sense(&existing, &list(&["死心", "明天"])),
            SenseMatch::Conflict(vec![0, 1])
        );
    }
}
