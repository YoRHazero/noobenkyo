use chrono::{DateTime, FixedOffset};
use dictionary_word::ContentProgress;

use dictionary_word::rules::text_list::clean_list;

/// Cleaned glosses, or `None` when none are left.
pub fn glosses(raw: Option<Vec<String>>) -> Option<Vec<String>> {
    raw.map(clean_list).filter(|glosses| !glosses.is_empty())
}

pub fn labels(raw: Option<Vec<String>>) -> Vec<String> {
    clean_list(raw.unwrap_or_default())
}

/// Progress, if any of its fields is given; missing counters are zero.
pub fn progress(
    stage: Option<u32>,
    passes: Option<u32>,
    lapses: Option<u32>,
    modified_at: Option<DateTime<FixedOffset>>,
) -> Option<ContentProgress> {
    if stage.is_none() && passes.is_none() && lapses.is_none() && modified_at.is_none() {
        return None;
    }
    Some(ContentProgress {
        stage: stage.unwrap_or(0),
        passes: passes.unwrap_or(0),
        lapses: lapses.unwrap_or(0),
        modified_at,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_glosses_count_as_none() {
        assert_eq!(glosses(Some(vec![" ".into()])), None);
        assert_eq!(glosses(None), None);
        assert_eq!(
            glosses(Some(vec!["放弃".into(), " 死心 ".into()])),
            Some(vec!["放弃".into(), "死心".into()])
        );
    }

    #[test]
    fn any_progress_field_makes_progress() {
        assert_eq!(progress(None, None, None, None), None);
        assert_eq!(
            progress(None, Some(3), None, None),
            Some(ContentProgress {
                stage: 0,
                passes: 3,
                lapses: 0,
                modified_at: None
            })
        );
    }
}
