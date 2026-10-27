use std::collections::VecDeque;

const KEPT: usize = 20;
const SHOWN: usize = 5;

/// The last lines a process wrote to stderr. When it fails without saying
/// why on the protocol (ssh refused, python missing), the reason is here.
#[derive(Default)]
pub(crate) struct StderrTail {
    lines: VecDeque<String>,
}

impl StderrTail {
    pub(crate) fn push(&mut self, line: String) {
        if line.trim().is_empty() {
            return;
        }
        if self.lines.len() == KEPT {
            self.lines.pop_front();
        }
        self.lines.push_back(line);
    }

    /// `：` and the last few lines, or nothing when it wrote nothing.
    pub(crate) fn summary(&self) -> String {
        if self.lines.is_empty() {
            return String::new();
        }
        let skip = self.lines.len().saturating_sub(SHOWN);
        let shown: Vec<&str> = self.lines.iter().skip(skip).map(String::as_str).collect();
        format!("：{}", shown.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_the_last_lines_that_say_something() {
        let mut tail = StderrTail::default();
        assert_eq!(tail.summary(), "");
        for i in 0..30 {
            tail.push(format!("line {i}"));
            tail.push("  ".into());
        }
        assert_eq!(
            tail.summary(),
            "：line 25\nline 26\nline 27\nline 28\nline 29"
        );
    }
}
