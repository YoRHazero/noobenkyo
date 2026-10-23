use listening_drama::ScriptLine;
use listening_drama::text::ruby::parse;
use serde_json::Value;

use crate::field::required_text;

/// The `number`-th line of a script (from 1), or every problem it has.
pub fn parse_line(number: usize, value: &Value) -> Result<ScriptLine, Vec<String>> {
    let Value::Object(object) = value else {
        return Err(vec![format!("第 {number} 行不是 JSON 对象")]);
    };
    let mut problems = Vec::new();
    let mut read = |name: &str| match required_text(object, name) {
        Ok(text) => Some(text),
        Err(problem) => {
            problems.push(format!("第 {number} 行：{problem}"));
            None
        }
    };
    let (speaker, ja, zh) = (read("speaker"), read("ja"), read("zh"));
    if let Some(Err(error)) = ja.as_deref().map(parse) {
        problems.push(format!("第 {number} 行 ja：{error}"));
    }
    match (speaker, ja, zh) {
        (Some(speaker), Some(ja), Some(zh)) if problems.is_empty() => {
            Ok(ScriptLine { speaker, ja, zh })
        }
        _ => Err(problems),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn reads_and_trims_the_fields() {
        let line = json!({ "speaker": " 美咲 ", "ja": " {明日|あした}ね。", "zh": "明天见。 ", "voice": "x" });
        assert_eq!(
            parse_line(1, &line),
            Ok(ScriptLine {
                speaker: "美咲".into(),
                ja: "{明日|あした}ね。".into(),
                zh: "明天见。".into(),
            })
        );
    }

    #[test]
    fn lists_every_problem_of_the_line() {
        let line = json!({ "speaker": "", "zh": 3 });
        assert_eq!(
            parse_line(2, &line),
            Err(vec![
                "第 2 行：speaker 为空".into(),
                "第 2 行：缺少 ja".into(),
                "第 2 行：zh 不是字符串".into(),
            ])
        );
        assert_eq!(
            parse_line(3, &json!("美咲")),
            Err(vec!["第 3 行不是 JSON 对象".into()])
        );
    }

    #[test]
    fn checks_the_reading_marks() {
        let line = json!({ "speaker": "美咲", "ja": "{明日|}ね。", "zh": "明天见。" });
        assert_eq!(
            parse_line(4, &line),
            Err(vec!["第 4 行 ja：读音标记 {明日|} 的读音为空".into()])
        );
    }
}
