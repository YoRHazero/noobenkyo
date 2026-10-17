/// Secondary glosses after a merge: the existing ones, then incoming glosses
/// not yet present. The primary gloss never changes.
pub fn merge_secondary(primary: &str, secondary: &[String], incoming: &[String]) -> Vec<String> {
    let mut merged = secondary.to_vec();
    for gloss in incoming {
        if gloss != primary && !merged.contains(gloss) {
            merged.push(gloss.clone());
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_new_glosses_only() {
        let secondary = vec!["死心".to_owned()];
        let incoming = vec!["断念".to_owned(), "放弃".to_owned(), "死心".to_owned()];
        assert_eq!(
            merge_secondary("放弃", &secondary, &incoming),
            ["死心", "断念"]
        );
    }
}
