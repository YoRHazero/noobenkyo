use crate::Role;

/// The role with its description trimmed; `None` when a main character's
/// description is blank.
pub fn clean_role(role: &Role) -> Option<Role> {
    match role {
        Role::Main { description } => {
            let description = description.trim();
            (!description.is_empty()).then(|| Role::Main {
                description: description.to_owned(),
            })
        }
        other => Some(other.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn main(description: &str) -> Role {
        Role::Main {
            description: description.into(),
        }
    }

    #[test]
    fn main_characters_need_a_description() {
        assert_eq!(clean_role(&main(" 高中生 ")), Some(main("高中生")));
        assert_eq!(clean_role(&main(" ")), None);
        assert_eq!(clean_role(&Role::Narrator), Some(Role::Narrator));
        assert_eq!(clean_role(&Role::Minor), Some(Role::Minor));
    }
}
