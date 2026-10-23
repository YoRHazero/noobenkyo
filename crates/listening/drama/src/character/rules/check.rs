use super::age::age_problem;
use super::name::check_name;
use super::references::references_problems;
use crate::{Gender, Language, NewCharacter};

/// A new character that passed the checks, with every text trimmed.
#[derive(Debug, PartialEq, Eq)]
pub struct CheckedCharacter<'a> {
    pub name: String,
    pub kana: String,
    pub gender: Gender,
    pub age: u32,
    pub prompt: Option<String>,
    pub references: Vec<CheckedReference<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CheckedReference<'a> {
    pub language: Language,
    pub audio: &'a [u8],
    pub text: String,
}

/// Checks a new character; any problem rejects it, and all of them are
/// listed. A blank prompt counts as none.
pub fn check_character(new: &NewCharacter) -> Result<CheckedCharacter<'_>, Vec<String>> {
    let mut problems = Vec::new();
    let names = match check_name(&new.name, &new.kana) {
        Ok(names) => Some(names),
        Err(found) => {
            problems.extend(found);
            None
        }
    };
    problems.extend(age_problem(new.age));
    problems.extend(references_problems(&new.references));
    let Some((name, kana)) = names.filter(|_| problems.is_empty()) else {
        return Err(problems);
    };
    Ok(CheckedCharacter {
        name,
        kana,
        gender: new.gender,
        age: new.age,
        prompt: new
            .prompt
            .as_deref()
            .map(str::trim)
            .filter(|prompt| !prompt.is_empty())
            .map(str::to_owned),
        references: new
            .references
            .iter()
            .map(|reference| CheckedReference {
                language: reference.language,
                audio: &reference.audio,
                text: reference.text.trim().to_owned(),
            })
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NewReference;

    fn new_character() -> NewCharacter {
        NewCharacter {
            name: " 美咲 ".into(),
            kana: "みさき".into(),
            gender: Gender::Female,
            age: 20,
            prompt: Some("  ".into()),
            references: vec![NewReference {
                language: Language::Ja,
                audio: b"wav".to_vec(),
                text: " こんにちは ".into(),
            }],
        }
    }

    #[test]
    fn trims_and_drops_a_blank_prompt() {
        let new = new_character();
        let checked = check_character(&new).unwrap();
        assert_eq!(checked.name, "美咲");
        assert_eq!(checked.prompt, None);
        assert_eq!(checked.references[0].text, "こんにちは");
        assert_eq!(checked.references[0].audio, b"wav");
    }

    #[test]
    fn lists_problems_of_every_field() {
        let new = NewCharacter {
            kana: "misaki".into(),
            age: 0,
            references: vec![],
            ..new_character()
        };
        assert_eq!(
            check_character(&new).unwrap_err(),
            vec![
                "名字的读音只能有假名，不能有 m",
                "年龄要在 1～100 之间",
                "至少要有一段参考音"
            ]
        );
    }
}
