//! The report of one import: what the text gave, what merging changed.

use dictionary_word::{MergeOutcome, SenseOutcome};
use serde::Serialize;

use crate::{Dropped, IgnoredField, Problem, Selected};

/// Summary of one import, shown to the user.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, specta::Type)]
pub struct ImportReport {
    /// Non-empty lines in the text.
    pub lines: u32,
    pub words_added: u32,
    pub readings_added: u32,
    pub senses_added: u32,
    pub senses_merged: u32,
    pub sentences_added: u32,
    pub progress_updated: u32,
    pub skipped_katakana: u32,
    /// What was dropped, by line.
    pub problems: Vec<Problem>,
    pub ignored_fields: Vec<IgnoredField>,
}

/// Combines the selected entries with what merging them changed; `outcomes`
/// are in the order of `selected.entries`.
pub fn report(selected: Selected, outcomes: &[MergeOutcome]) -> ImportReport {
    let mut report = ImportReport {
        lines: selected.lines,
        skipped_katakana: selected.skipped_katakana,
        problems: selected.problems,
        ignored_fields: selected.ignored_fields,
        ..ImportReport::default()
    };
    for (entry, outcome) in selected.entries.iter().zip(outcomes) {
        tally(&mut report, outcome);
        if let SenseOutcome::Conflict(primaries) = &outcome.sense {
            report.problems.push(Problem {
                line: entry.line,
                writing: entry.writing.clone(),
                dropped: Dropped::Sense,
                reason: format!("释义和多个已有义项有交集：{}", primaries.join("、")),
            });
        }
    }
    report.problems.sort_by_key(|problem| problem.line);
    report
}

fn tally(report: &mut ImportReport, outcome: &MergeOutcome) {
    report.words_added += u32::from(outcome.word_added);
    report.readings_added += u32::from(outcome.reading_added);
    report.senses_added += u32::from(outcome.sense == SenseOutcome::Added);
    report.senses_merged += u32::from(outcome.sense == SenseOutcome::Merged);
    report.sentences_added += outcome.sentences_added;
    report.progress_updated += u32::from(outcome.progress_updated);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ImportOptions, convert, select};

    #[test]
    fn adds_up_outcomes_and_reports_conflicts_by_line() {
        let text = concat!(
            r#"{"writing": "明日", "kana": "あした", "glosses": ["明天"], "sentences": [{"marked": "[[{明日|あした}]]"}, {"marked": "明日"}]}"#,
            "\n",
            r#"{"writing": "上", "kana": "うえ", "glosses": ["上面", "上级"]}"#,
        );
        let selected = select(convert(text), ImportOptions::default());
        let outcomes = [
            MergeOutcome {
                word_added: true,
                reading_added: true,
                sense: SenseOutcome::Added,
                sentences_added: 1,
                progress_updated: false,
            },
            MergeOutcome {
                sense: SenseOutcome::Conflict(vec!["上面".into(), "上级".into()]),
                ..MergeOutcome::default()
            },
        ];
        let report = report(selected, &outcomes);
        assert_eq!(report.lines, 2);
        assert_eq!(
            (
                report.words_added,
                report.readings_added,
                report.senses_added
            ),
            (1, 1, 1)
        );
        assert_eq!(report.sentences_added, 1);
        let problems: Vec<(u32, &Dropped)> = report
            .problems
            .iter()
            .map(|p| (p.line, &p.dropped))
            .collect();
        assert_eq!(
            problems,
            [
                (
                    1,
                    &Dropped::Sentence {
                        marked: "明日".into()
                    }
                ),
                (2, &Dropped::Sense),
            ]
        );
        assert_eq!(
            report.problems[1].reason,
            "释义和多个已有义项有交集：上面、上级"
        );
    }
}
