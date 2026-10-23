/// The trimmed name of a world; `None` when it is blank.
pub fn clean_name(raw: &str) -> Option<String> {
    let name = raw.trim();
    (!name.is_empty()).then(|| name.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_and_rejects_blank() {
        assert_eq!(clean_name(" 星见町 "), Some("星见町".into()));
        assert_eq!(clean_name(" "), None);
    }
}
