use listening_drama::Script;
use serde_json::Value;

use crate::field::required_text;
use crate::line::parse_line;

/// Parses the standard script JSON. It passes or fails as a whole and is
/// never repaired; only surrounding whitespace is removed from each text.
/// Fields it does not know are ignored. Fails with every problem found.
pub fn parse_script(text: &str) -> Result<Script, Vec<String>> {
    let value: Value =
        serde_json::from_str(text).map_err(|error| vec![format!("不是 JSON：{error}")])?;
    let Value::Object(object) = value else {
        return Err(vec!["不是 JSON 对象".to_owned()]);
    };
    let mut problems = Vec::new();
    let mut read = |name: &str| match required_text(&object, name) {
        Ok(text) => Some(text),
        Err(problem) => {
            problems.push(problem);
            None
        }
    };
    let (title, summary) = (read("title"), read("summary"));
    let mut lines = Vec::new();
    match object.get("lines") {
        None => problems.push("缺少 lines".to_owned()),
        Some(Value::Array(items)) if items.is_empty() => problems.push("lines 没有行".to_owned()),
        Some(Value::Array(items)) => {
            for (index, item) in items.iter().enumerate() {
                match parse_line(index + 1, item) {
                    Ok(line) => lines.push(line),
                    Err(found) => problems.extend(found),
                }
            }
        }
        Some(_) => problems.push("lines 不是数组".to_owned()),
    }
    match (title, summary) {
        (Some(title), Some(summary)) if problems.is_empty() => Ok(Script {
            title,
            summary,
            lines,
        }),
        _ => Err(problems),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use listening_drama::ScriptLine;
    use serde_json::json;

    fn parse(value: Value) -> Result<Script, Vec<String>> {
        parse_script(&value.to_string())
    }

    #[test]
    fn parses_the_sample_script() {
        let script = parse_script(include_str!("../../../../samples/script.json")).unwrap();
        assert_eq!(script.title, "海边的早晨");
        assert_eq!(script.lines[0].speaker, "旁白");
        assert!(script.lines.len() > 5);
    }

    #[test]
    fn parses_and_ignores_unknown_fields() {
        let script = parse(json!({
            "title": " 初遇 ",
            "summary": "两人相遇。",
            "notes": "多余的字段",
            "lines": [{ "speaker": "旁白", "ja": "{朝|あさ}。", "zh": "早晨。" }]
        }))
        .unwrap();
        assert_eq!(
            script,
            Script {
                title: "初遇".into(),
                summary: "两人相遇。".into(),
                lines: vec![ScriptLine {
                    speaker: "旁白".into(),
                    ja: "{朝|あさ}。".into(),
                    zh: "早晨。".into(),
                }],
            }
        );
    }

    #[test]
    fn rejects_what_is_not_a_json_object() {
        assert!(parse_script("```json\n{}\n```").unwrap_err()[0].starts_with("不是 JSON："));
        assert_eq!(parse(json!([1])).unwrap_err(), vec!["不是 JSON 对象"]);
    }

    #[test]
    fn lists_every_problem() {
        let problems = parse(json!({
            "title": "",
            "lines": [
                { "speaker": "旁白", "ja": "あ", "zh": "啊" },
                { "speaker": "美咲", "ja": "{明日|asita}", "zh": "" }
            ]
        }))
        .unwrap_err();
        assert_eq!(
            problems,
            vec![
                "title 为空",
                "缺少 summary",
                "第 2 行：zh 为空",
                "第 2 行 ja：读音标记 {明日|asita} 的读音只能有假名，不能有 a",
            ]
        );
    }

    #[test]
    fn needs_at_least_one_line() {
        let base = json!({ "title": "初遇", "summary": "两人相遇。" });
        let with = |lines: Value| {
            let mut value = base.clone();
            value["lines"] = lines;
            parse(value).unwrap_err()
        };
        assert_eq!(parse(base.clone()).unwrap_err(), vec!["缺少 lines"]);
        assert_eq!(with(json!([])), vec!["lines 没有行"]);
        assert_eq!(with(json!("台词")), vec!["lines 不是数组"]);
    }
}
