use serde_json::{Map, Value};

/// A required string field with surrounding whitespace removed, or why it
/// can't be read: missing, not a string, or blank.
pub fn required_text(object: &Map<String, Value>, name: &str) -> Result<String, String> {
    match object.get(name) {
        None => Err(format!("缺少 {name}")),
        Some(Value::String(text)) => {
            let text = text.trim();
            if text.is_empty() {
                Err(format!("{name} 为空"))
            } else {
                Ok(text.to_owned())
            }
        }
        Some(_) => Err(format!("{name} 不是字符串")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn object(value: Value) -> Map<String, Value> {
        value.as_object().unwrap().clone()
    }

    #[test]
    fn trims_the_text() {
        let found = object(json!({ "title": " 初遇 " }));
        assert_eq!(required_text(&found, "title"), Ok("初遇".into()));
    }

    #[test]
    fn says_why_it_cannot_be_read() {
        let found = object(json!({ "summary": " ", "lines": 3, "zh": null }));
        assert_eq!(required_text(&found, "title"), Err("缺少 title".into()));
        assert_eq!(required_text(&found, "summary"), Err("summary 为空".into()));
        assert_eq!(
            required_text(&found, "lines"),
            Err("lines 不是字符串".into())
        );
        assert_eq!(required_text(&found, "zh"), Err("zh 不是字符串".into()));
    }
}
