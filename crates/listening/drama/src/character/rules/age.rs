pub const MIN_AGE: u32 = 1;
pub const MAX_AGE: u32 = 100;

/// What is wrong with an age, if anything.
pub fn age_problem(age: u32) -> Option<String> {
    (!(MIN_AGE..=MAX_AGE).contains(&age)).then(|| format!("年龄要在 {MIN_AGE}～{MAX_AGE} 之间"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_one_to_a_hundred() {
        assert_eq!(age_problem(1), None);
        assert_eq!(age_problem(100), None);
        assert!(age_problem(0).is_some());
        assert!(age_problem(101).is_some());
    }
}
