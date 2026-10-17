/// Trims every item, drops empty ones and duplicates, keeping the first occurrence.
pub fn clean_list(items: Vec<String>) -> Vec<String> {
    let mut cleaned: Vec<String> = Vec::with_capacity(items.len());
    for item in items {
        let item = item.trim();
        if !item.is_empty() && !cleaned.iter().any(|kept| kept == item) {
            cleaned.push(item.to_owned());
        }
    }
    cleaned
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_and_dedupes_in_order() {
        let items = vec![" 放弃".into(), "".into(), "死心".into(), "放弃 ".into()];
        assert_eq!(clean_list(items), ["放弃", "死心"]);
    }
}
