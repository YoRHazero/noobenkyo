use crate::text::kana::is_kana;
use crate::text::kanji::has_kanji;

/// The trimmed name and reading, or what is wrong with them. The reading is
/// kana only; a name without kanji is its own reading.
pub fn check_name(name: &str, kana: &str) -> Result<(String, String), Vec<String>> {
    let (name, kana) = (name.trim(), kana.trim());
    let mut problems = Vec::new();
    if name.is_empty() {
        problems.push("名字为空".to_owned());
    }
    if kana.is_empty() {
        problems.push("名字的读音为空".to_owned());
    } else if let Some(found) = kana.chars().find(|c| !is_kana(*c)) {
        problems.push(format!("名字的读音只能有假名，不能有 {found}"));
    }
    if !name.is_empty() && !kana.is_empty() && !has_kanji(name) && kana != name {
        problems.push("名字里没有汉字，读音要和名字相同".to_owned());
    }
    if problems.is_empty() {
        Ok((name.to_owned(), kana.to_owned()))
    } else {
        Err(problems)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_name_and_reading() {
        assert_eq!(
            check_name(" 美咲 ", " みさき "),
            Ok(("美咲".into(), "みさき".into()))
        );
        assert_eq!(
            check_name("アリス・リデル", "アリス・リデル"),
            Ok(("アリス・リデル".into(), "アリス・リデル".into()))
        );
    }

    #[test]
    fn reading_is_kana_only() {
        assert_eq!(
            check_name("", " "),
            Err(vec!["名字为空".into(), "名字的读音为空".into()])
        );
        assert_eq!(
            check_name("美咲", "misaki"),
            Err(vec!["名字的读音只能有假名，不能有 m".into()])
        );
    }

    #[test]
    fn name_without_kanji_is_its_own_reading() {
        assert_eq!(
            check_name("ミサキ", "みさき"),
            Err(vec!["名字里没有汉字，读音要和名字相同".into()])
        );
        assert_eq!(
            check_name("Alice", "Alice"),
            Err(vec!["名字的读音只能有假名，不能有 A".into()])
        );
    }
}
