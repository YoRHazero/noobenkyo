/// An episode's number, title and summary as they will be stored, or what
/// is wrong with them. The number may be any finite number.
pub fn check_fields(
    number: f64,
    title: &str,
    summary: &str,
) -> Result<(f64, String, String), Vec<String>> {
    let (title, summary) = (title.trim(), summary.trim());
    let mut problems = Vec::new();
    if !number.is_finite() {
        problems.push("集数不是有限的数".to_owned());
    }
    if title.is_empty() {
        problems.push("标题为空".to_owned());
    }
    if summary.is_empty() {
        problems.push("梗概为空".to_owned());
    }
    if problems.is_empty() {
        Ok((number, title.to_owned(), summary.to_owned()))
    } else {
        Err(problems)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_title_and_summary() {
        assert_eq!(
            check_fields(1.5, " 初遇 ", " 两人相遇。 "),
            Ok((1.5, "初遇".into(), "两人相遇。".into()))
        );
    }

    #[test]
    fn lists_every_problem() {
        assert_eq!(
            check_fields(f64::NAN, " ", ""),
            Err(vec![
                "集数不是有限的数".into(),
                "标题为空".into(),
                "梗概为空".into()
            ])
        );
    }
}
