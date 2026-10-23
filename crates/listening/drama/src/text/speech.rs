use super::ruby::{RubyError, RubySegment, parse};

/// The text handed to text-to-speech: every reading mark replaced by its reading.
pub fn speech_text(ja: &str) -> Result<String, RubyError> {
    let segments = parse(ja)?;
    Ok(segments
        .into_iter()
        .map(|segment| match segment {
            RubySegment::Text { text } => text,
            RubySegment::Ruby { reading, .. } => reading,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_marks_with_readings() {
        assert_eq!(
            speech_text("{明日|あした}、{美咲|みさき}と会う。").unwrap(),
            "あした、みさきと会う。"
        );
        assert_eq!(speech_text("会う。").unwrap(), "会う。");
    }

    #[test]
    fn fails_on_malformed_marks() {
        assert!(speech_text("{明日|あした").is_err());
    }
}
