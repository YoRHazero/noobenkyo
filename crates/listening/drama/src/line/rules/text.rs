use crate::Language;
use crate::text::ruby::parse;

/// One language of a line, trimmed, or what is wrong with it: it is not
/// empty, and Japanese has well-formed reading marks.
pub fn check_text(language: Language, raw: &str) -> Result<String, Vec<String>> {
    let text = raw.trim();
    if text.is_empty() {
        return Err(vec![format!("{}为空", language.name())]);
    }
    if language == Language::Ja
        && let Err(error) = parse(text)
    {
        return Err(vec![format!("{}：{error}", language.name())]);
    }
    Ok(text.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_the_text() {
        assert_eq!(
            check_text(Language::Ja, " {明日|あした}ね。 "),
            Ok("{明日|あした}ね。".into())
        );
        assert_eq!(
            check_text(Language::Zh, " 明天见。 "),
            Ok("明天见。".into())
        );
    }

    #[test]
    fn rejects_blank_text() {
        assert_eq!(check_text(Language::Zh, " "), Err(vec!["中文为空".into()]));
    }

    #[test]
    fn checks_reading_marks_in_japanese_only() {
        assert_eq!(
            check_text(Language::Ja, "{明日|}ね。"),
            Err(vec!["日语：读音标记 {明日|} 的读音为空".into()])
        );
        assert_eq!(check_text(Language::Zh, "{明日|}"), Ok("{明日|}".into()));
    }
}
