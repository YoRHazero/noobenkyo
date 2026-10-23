/// What a cast lacks to write a script with: a narrator and at least one
/// main character. `None` when it lacks nothing.
pub fn missing_roles(has_narrator: bool, mains: usize) -> Option<String> {
    let mut missing = Vec::new();
    if !has_narrator {
        missing.push("旁白");
    }
    if mains == 0 {
        missing.push("主要角色");
    }
    (!missing.is_empty()).then(|| missing.join("、"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn needs_a_narrator_and_a_main_character() {
        assert_eq!(missing_roles(true, 1), None);
        assert_eq!(missing_roles(false, 2), Some("旁白".into()));
        assert_eq!(missing_roles(true, 0), Some("主要角色".into()));
        assert_eq!(missing_roles(false, 0), Some("旁白、主要角色".into()));
    }
}
