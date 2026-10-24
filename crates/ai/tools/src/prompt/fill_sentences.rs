use dictionary::ProgressState;
use serde::Deserialize;

use super::shell_word::shell_word;

/// Sentences to write for senses that have none, from the rows the
/// dictionary page's keyword and progress filter show.
#[derive(Clone, Debug, Deserialize, specta::Type)]
pub struct FillSentencesTask {
    /// Blank for no keyword.
    pub keyword: Option<String>,
    pub progress: ProgressState,
    /// How many rows to take, in the page's order.
    pub limit: u32,
    /// How many sentences each sense gets.
    pub per_sense: u32,
}

/// The prompt for filling in sentences and sending the rows to the app.
/// The rows are given as the export command that yields them, so they are
/// exactly the ones the user chose.
pub fn fill_sentences_prompt(task: &FillSentencesTask) -> String {
    let mut export = vec!["noobenkyo dictionary export --completeness no-sentence".to_owned()];
    if let Some(keyword) = task.keyword.as_deref().map(str::trim)
        && !keyword.is_empty()
    {
        export.push(format!("--keyword {}", shell_word(keyword)));
    }
    match task.progress {
        ProgressState::Any => {}
        ProgressState::Untouched => export.push("--progress untouched".to_owned()),
        ProgressState::Touched => export.push("--progress touched".to_owned()),
    }
    export.push(format!("--limit {}", task.limit));
    format!(
        "用 noobenkyo 命令行给词库里还没有例句的义项补例句，送进 app。用法先运行 `noobenkyo dictionary guide` 看。\n\n要补的行：`{}`\n每个义项补 {} 条例句，其余字段不改。",
        export.join(" "),
        task.per_sense
    )
}

#[cfg(test)]
mod tests {
    use dictionary::ProgressState;

    use super::{FillSentencesTask, fill_sentences_prompt};

    #[test]
    fn the_rows_are_the_export_of_the_filter() {
        let prompt = fill_sentences_prompt(&FillSentencesTask {
            keyword: Some(" 猫 ".into()),
            progress: ProgressState::Any,
            limit: 20,
            per_sense: 2,
        });
        assert_eq!(
            prompt,
            "用 noobenkyo 命令行给词库里还没有例句的义项补例句，送进 app。用法先运行 `noobenkyo dictionary guide` 看。

要补的行：`noobenkyo dictionary export --completeness no-sentence --keyword 猫 --limit 20`
每个义项补 2 条例句，其余字段不改。"
        );
    }

    #[test]
    fn a_blank_keyword_is_left_out_and_progress_is_kept() {
        let prompt = fill_sentences_prompt(&FillSentencesTask {
            keyword: Some("  ".into()),
            progress: ProgressState::Untouched,
            limit: 5,
            per_sense: 1,
        });
        assert!(prompt.contains(
            "`noobenkyo dictionary export --completeness no-sentence --progress untouched --limit 5`"
        ));
    }
}
